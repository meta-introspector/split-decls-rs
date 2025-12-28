macro_rules! deps {
    () => {
        CtfeValidationMode!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl CtfeValidationMode { fn allow_immutable_unsafe_cell (self) -> bool { match self { CtfeValidationMode :: Static { .. } => false , CtfeValidationMode :: Promoted { .. } => false , CtfeValidationMode :: Const { allow_immutable_unsafe_cell , .. } => { allow_immutable_unsafe_cell } } } }
    };
}

impl_347!()