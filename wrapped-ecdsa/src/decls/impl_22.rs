macro_rules! deps {
    () => {
        Signature!();
        MaxSize!();
        MaxOverhead!();
        EcdsaCurve!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < C > Clone for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn clone (& self) -> Self { Self { bytes : self . bytes . clone () , r_range : self . r_range . clone () , s_range : self . s_range . clone () , } } }
    };
}

impl_22!()