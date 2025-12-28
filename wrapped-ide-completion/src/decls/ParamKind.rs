macro_rules! ParamKind {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum ParamKind { Function (ast :: Fn) , Closure (ast :: ClosureExpr) , }
    };
}

ParamKind!();