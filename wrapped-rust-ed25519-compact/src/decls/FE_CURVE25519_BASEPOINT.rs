macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! FE_CURVE25519_BASEPOINT {
    () => {
        deps!();
        # [cfg (feature = "x25519")] pub (crate) static FE_CURVE25519_BASEPOINT : Fe = Fe ([9 , 0 , 0 , 0 , 0]) ;
    };
}

FE_CURVE25519_BASEPOINT!();