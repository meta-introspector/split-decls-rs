macro_rules! AlternativeExprs {
    () => {
        # [doc = " Helper enum to squash big number of alternative trees into `Many` variant as there is too many"] # [doc = " to take into account."] # [derive (Debug)] enum AlternativeExprs < 'db > { # [doc = " There are few trees, so we keep track of them all"] Few (FxHashSet < Expr < 'db > >) , # [doc = " There are too many trees to keep track of"] Many , }
    };
}

AlternativeExprs!()