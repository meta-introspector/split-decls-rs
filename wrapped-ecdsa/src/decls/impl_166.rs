macro_rules! deps {
    () => {
        EcdsaCurve!();
        Signature!();
        MaxOverhead!();
        SignatureWithOid!();
        MaxSize!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        # [cfg (all (feature = "der" , feature = "digest"))] impl < C > From < SignatureWithOid < C > > for der :: Signature < C > where C : EcdsaCurve , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn from (sig : SignatureWithOid < C >) -> der :: Signature < C > { sig . to_der () } }
    };
}

impl_166!();