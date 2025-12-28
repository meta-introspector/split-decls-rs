macro_rules! ParamTerm {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum ParamTerm { Ty (ty :: ParamTy) , Const (ty :: ParamConst) , }
    };
}

ParamTerm!()