use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn fatally_break_rust(tcx: TyCtxt<'_>, span: Span) -> ! {
    let dcx = tcx.dcx();
    let mut diag = dcx
        .struct_span_bug(
            span,
            "It looks like you're trying to break rust; would you like some ICE?",
        );
    diag.note("the compiler expectedly panicked. this is a feature.");
    diag.note(
        "we would appreciate a joke overview: \
         https://github.com/rust-lang/rust/issues/43162#issuecomment-320764675",
    );
    diag.note(
        format!("rustc {} running on {}", tcx.sess.cfg_version, config::host_tuple(),),
    );
    if let Some((flags, excluded_cargo_defaults)) = rustc_session::utils::extra_compiler_flags() {
        diag.note(format!("compiler flags: {}", flags.join(" ")));
        if excluded_cargo_defaults {
            diag.note("some of the compiler flags provided by cargo are hidden");
        }
    }
    diag.emit()
}
