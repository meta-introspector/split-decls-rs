macro_rules! MismatchedArgCount {
    () => {
        # [derive (Debug)] pub struct MismatchedArgCount { pub call_expr : InFile < ExprOrPatPtr > , pub expected : usize , pub found : usize , }
    };
}

MismatchedArgCount!();