macro_rules! deps {
    () => {
        LocalRef!();
        OperandRef!();
    };
}

macro_rules! impl_529 {
    () => {
        deps!();
        impl < 'tcx , V : CodegenObject > LocalRef < 'tcx , V > { fn new_operand (layout : TyAndLayout < 'tcx >) -> LocalRef < 'tcx , V > { if layout . is_zst () { LocalRef :: Operand (OperandRef :: zero_sized (layout)) } else { LocalRef :: PendingOperand } } }
    };
}

impl_529!()