macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! PublicKey {
    () => {
        deps!();
        # [doc = " A public key."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct PublicKey ([u8 ; POINT_BYTES]) ;
    };
}

PublicKey!();