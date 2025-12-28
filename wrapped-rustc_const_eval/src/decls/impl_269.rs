macro_rules! deps {
    () => {
        Immediate!();
        ImmTy!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > std :: ops :: Deref for ImmTy < 'tcx , Prov > { type Target = Immediate < Prov > ; # [inline (always)] fn deref (& self) -> & Immediate < Prov > { & self . imm } }
    };
}

impl_269!()