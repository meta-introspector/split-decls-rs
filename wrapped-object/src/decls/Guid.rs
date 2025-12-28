macro_rules! Guid {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [repr (C)] pub struct Guid (pub [u8 ; 16]) ;
    };
}

Guid!();