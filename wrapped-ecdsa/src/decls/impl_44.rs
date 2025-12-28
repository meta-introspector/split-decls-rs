macro_rules! deps {
    () => {
        EcdsaCurve!();
        SignatureWithOid!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        # [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl < C > DynAssociatedAlgorithmIdentifier for SignatureWithOid < C > where C : EcdsaCurve , { fn algorithm_identifier (& self) -> spki :: Result < AlgorithmIdentifierOwned > { Ok (AlgorithmIdentifierOwned { oid : self . oid , parameters : None , }) } }
    };
}

impl_44!()