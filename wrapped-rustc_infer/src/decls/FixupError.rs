macro_rules! deps {
    () => {
        TyOrConstInferVar!();
    };
}

macro_rules! FixupError {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] pub struct FixupError { unresolved : TyOrConstInferVar , }
    };
}

FixupError!()