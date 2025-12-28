macro_rules! Digest {
    () => {
        # [doc = " A digest."] # [derive (Clone , Copy , Eq , Hash , PartialEq)] pub struct Digest (pub [u8 ; 16]) ;
    };
}

Digest!()