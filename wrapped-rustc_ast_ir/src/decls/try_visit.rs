macro_rules! deps {
    () => {
        VisitorResult!();
    };
}

macro_rules! try_visit {
    () => {
        deps!();
        # [macro_export] macro_rules ! try_visit { ($ e : expr) => { match $ crate :: visit :: VisitorResult :: branch ($ e) { core :: ops :: ControlFlow :: Continue (()) => () , # [allow (unreachable_code)] core :: ops :: ControlFlow :: Break (r) => { return $ crate :: visit :: VisitorResult :: from_residual (r) ; } } } ; }
    };
}

try_visit!()