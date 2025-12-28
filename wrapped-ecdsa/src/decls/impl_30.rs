macro_rules! deps {
    () => {
        EcdsaCurve!();
        Signature!();
        MaxSize!();
        MaxOverhead!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < C > From < Signature < C > > for Box < [u8] > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn from (signature : Signature < C >) -> Box < [u8] > { signature . to_vec () . into_boxed_slice () } }
    };
}

impl_30!();