// Generated macro for macro_3948 (macro)
macro_rules! Depcrate_loopsmacro_3948 {
() => {
// Module: crate::loops
// Provides: {"macro_3948"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for empty `loop` expressions."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " These busy loops burn CPU cycles without doing"] # [doc = " anything. It is _almost always_ a better idea to `panic!` than to have"] # [doc = " a busy loop."] # [doc = ""] # [doc = " If panicking isn't possible, think of the environment and either:"] # [doc = "   - block on something"] # [doc = "   - sleep the thread for some microseconds"] # [doc = "   - yield or pause the thread"] # [doc = ""] # [doc = " For `std` targets, this can be done with"] # [doc = " [`std::thread::sleep`](https://doc.rust-lang.org/std/thread/fn.sleep.html)"] # [doc = " or [`std::thread::yield_now`](https://doc.rust-lang.org/std/thread/fn.yield_now.html)."] # [doc = ""] # [doc = " For `no_std` targets, doing this is more complicated, especially because"] # [doc = " `#[panic_handler]`s can't panic. To stop/pause the thread, you will"] # [doc = " probably need to invoke some target-specific intrinsic. Examples include:"] # [doc = "   - [`x86_64::instructions::hlt`](https://docs.rs/x86_64/0.12.2/x86_64/instructions/fn.hlt.html)"] # [doc = "   - [`cortex_m::asm::wfi`](https://docs.rs/cortex-m/0.6.3/cortex_m/asm/fn.wfi.html)"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " loop {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub EMPTY_LOOP , suspicious , "empty `loop {}`, which should block or sleep" }
};
}
