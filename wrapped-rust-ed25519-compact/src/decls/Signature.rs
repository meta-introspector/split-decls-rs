macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! Signature {
    () => {
        deps!();
        # [doc = " An Ed25519 signature."] # [derive (Copy , Clone , Eq , PartialEq , Hash)] pub struct Signature ([u8 ; Signature :: BYTES]) ;
    };
}

Signature!()