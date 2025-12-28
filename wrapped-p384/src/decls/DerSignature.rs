macro_rules! deps {
    () => {
        Signature!();
        NistP384!();
    };
}

macro_rules! DerSignature {
    () => {
        deps!();
        # [doc = " ECDSA/P-384 signature (ASN.1 DER encoded)"] pub type DerSignature = ecdsa_core :: der :: Signature < NistP384 > ;
    };
}

DerSignature!()