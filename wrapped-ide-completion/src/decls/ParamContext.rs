macro_rules! deps {
    () => {
        ParamKind!();
    };
}

macro_rules! ParamContext {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub (crate) struct ParamContext { pub (crate) param_list : ast :: ParamList , pub (crate) param : ast :: Param , pub (crate) kind : ParamKind , }
    };
}

ParamContext!();