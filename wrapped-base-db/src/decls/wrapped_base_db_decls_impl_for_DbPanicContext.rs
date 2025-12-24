use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl DbPanicContext {
    pub fn enter(frame: String) -> DbPanicContext {
        #[expect(clippy::print_stderr, reason = "already panicking anyway")]
        fn set_hook() {
            let default_hook = panic::take_hook();
            panic::set_hook(
                Box::new(move |panic_info| {
                    default_hook(panic_info);
                    if let Some(backtrace) = salsa::Backtrace::capture() {
                        eprintln!("{backtrace:#}");
                    }
                    DbPanicContext::with_ctx(|ctx| {
                        if !ctx.is_empty() {
                            eprintln!("additional context:");
                            for (idx, frame) in ctx.iter().enumerate() {
                                eprintln!("{idx:>4}: {frame}\n");
                            }
                        }
                    });
                }),
            );
        }
        static SET_HOOK: Once = Once::new();
        SET_HOOK.call_once(set_hook);
        Self::with_ctx(|ctx| ctx.push(frame));
        DbPanicContext
    }
    fn with_ctx(f: impl FnOnce(&mut Vec<String>)) {
        thread_local! {
            static CTX : RefCell < Vec < String >> = const { RefCell::new(Vec::new()) };
        }
        CTX.with(|ctx| f(&mut ctx.borrow_mut()));
    }
}
