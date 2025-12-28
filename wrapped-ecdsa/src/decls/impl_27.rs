macro_rules! deps {
    () => {
        Signature!();
        EcdsaCurve!();
        MaxSize!();
        MaxOverhead!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < C > From < crate :: Signature < C > > for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn from (sig : crate :: Signature < C >) -> Signature < C > { sig . to_der () } }
    };
}

impl_27!()