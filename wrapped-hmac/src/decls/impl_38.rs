macro_rules! deps {
    () => {
        SimpleHmacReset!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < D : Digest + BlockSizeUser > KeyInit for SimpleHmacReset < D > { fn new (key : & Key < Self >) -> Self { Self :: new_from_slice (key . as_slice ()) . unwrap () } # [inline] fn new_from_slice (key : & [u8]) -> Result < Self , InvalidLength > { let der_key = get_der_key :: < D > (key) ; let mut ipad_key = der_key . clone () ; ipad_key . iter_mut () . for_each (| b : & mut u8 | * b ^= IPAD) ; let mut digest = D :: new () ; digest . update (& ipad_key) ; let mut opad_key = der_key ; opad_key . iter_mut () . for_each (| b : & mut u8 | * b ^= OPAD) ; Ok (Self { digest , opad_key , ipad_key , }) } }
    };
}

impl_38!()