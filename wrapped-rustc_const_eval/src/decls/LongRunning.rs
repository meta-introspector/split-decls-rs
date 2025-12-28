macro_rules! LongRunning {
    () => {
        # [derive (LintDiagnostic)] # [diag (const_eval_long_running)] # [note] pub struct LongRunning { # [help] pub item_span : Span , }
    };
}

LongRunning!();