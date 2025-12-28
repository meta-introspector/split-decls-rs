macro_rules! deps {
    () => {
        Funclet!();
        OperandBundle!();
        OperandBundleBox!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < 'll > Funclet < 'll > { pub (crate) fn new (cleanuppad : & 'll Value) -> Self { Funclet { cleanuppad , operand : llvm :: OperandBundleBox :: new ("funclet" , & [cleanuppad]) } } pub (crate) fn cleanuppad (& self) -> & 'll Value { self . cleanuppad } pub (crate) fn bundle (& self) -> & llvm :: OperandBundle < 'll > { self . operand . as_ref () } }
    };
}

impl_185!();