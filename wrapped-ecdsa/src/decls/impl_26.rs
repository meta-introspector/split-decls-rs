macro_rules! deps {
    () => {
        MaxSize!();
        Signature!();
        MaxOverhead!();
        EcdsaCurve!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < C > FixedTag for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { const TAG : Tag = Tag :: Sequence ; }
    };
}

impl_26!();