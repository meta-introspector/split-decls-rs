macro_rules! LongRunningWarn {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_long_running)] pub struct LongRunningWarn { # [primary_span] # [label] pub span : Span , # [help] pub item_span : Span , pub force_duplicate : usize , }
    };
}

LongRunningWarn!();