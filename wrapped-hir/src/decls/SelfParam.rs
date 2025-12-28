macro_rules! SelfParam {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct SelfParam { func : FunctionId , }
    };
}

SelfParam!();