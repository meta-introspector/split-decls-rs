macro_rules! deps {
    () => {
        DiagCtxtInner!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl Drop for DiagCtxtInner { fn drop (& mut self) { self . emit_stashed_diagnostics () ; self . flush_delayed () ; if ! self . has_printed && ! self . suppressed_expected_diag && ! std :: thread :: panicking () { if let Some (backtrace) = & self . must_produce_diag { let suggestion = match backtrace . status () { BacktraceStatus :: Disabled => String :: from ("Backtraces are currently disabled: set `RUST_BACKTRACE=1` and re-run \
                        to see where it happened." ,) , BacktraceStatus :: Captured => format ! ("This happened in the following `must_produce_diag` call's backtrace:\n\
                        {backtrace}" ,) , _ => String :: from ("(impossible to capture backtrace where this happened)") , } ; panic ! ("`trimmed_def_paths` called, diagnostics were expected but none were emitted. \
                    Use `with_no_trimmed_paths` for debugging. {suggestion}") ; } } } }
    };
}

impl_44!()