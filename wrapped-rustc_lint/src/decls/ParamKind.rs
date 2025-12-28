macro_rules! ParamKind {
    () => {
        # [derive (PartialEq , Eq , Hash , Debug , Copy , Clone)] enum ParamKind { Early (Symbol , u32) , Free (DefId) , Late , }
    };
}

ParamKind!();