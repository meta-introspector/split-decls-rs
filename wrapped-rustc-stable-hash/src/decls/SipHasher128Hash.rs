macro_rules! deps {
    () => {
        SipHasher128!();
    };
}

macro_rules! SipHasher128Hash {
    () => {
        deps!();
        # [doc = " Hashing result of [`SipHasher128`]"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct SipHasher128Hash (pub [u64 ; 2]) ;
    };
}

SipHasher128Hash!();