macro_rules! deps {
    () => {
        MaxOverhead!();
        Signature!();
        SignatureWithOid!();
        MaxSize!();
        EcdsaCurve!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        # [cfg (all (feature = "der" , feature = "digest"))] impl < C > From < & SignatureWithOid < C > > for der :: Signature < C > where C : EcdsaCurve , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn from (sig : & SignatureWithOid < C >) -> der :: Signature < C > { sig . to_der () } }
    };
}

impl_167!()