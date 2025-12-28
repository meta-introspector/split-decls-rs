macro_rules! MismatchedTupleStructPatArgCount {
    () => {
        # [derive (Debug)] pub struct MismatchedTupleStructPatArgCount { pub expr_or_pat : InFile < ExprOrPatPtr > , pub expected : usize , pub found : usize , }
    };
}

MismatchedTupleStructPatArgCount!()