macro_rules! deps {
    () => {
        OpTy!();
        Operand!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > OpTy < 'tcx , Prov > { # [inline (always)] pub (super) fn op (& self) -> & Operand < Prov > { & self . op } }
    };
}

impl_277!();