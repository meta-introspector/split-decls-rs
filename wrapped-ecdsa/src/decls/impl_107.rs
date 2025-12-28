macro_rules! deps {
    () => {
        SignatureSize!();
        EcdsaCurve!();
        MaxSize!();
        Signature!();
        MaxOverhead!();
        VerifyingKey!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        # [cfg (feature = "der")] impl < C > PrehashVerifier < der :: Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn verify_prehash (& self , prehash : & [u8] , signature : & der :: Signature < C >) -> Result < () > { let signature = Signature :: < C > :: try_from (signature . clone ()) ? ; PrehashVerifier :: < Signature < C > > :: verify_prehash (self , prehash , & signature) } }
    };
}

impl_107!()