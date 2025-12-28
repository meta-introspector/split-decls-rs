macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! SecretKey {
    () => {
        deps!();
        # [doc = " A secret key."] # [derive (Clone , Debug , Eq , PartialEq , Hash)] pub struct SecretKey ([u8 ; SecretKey :: BYTES]) ;
    };
}

SecretKey!()