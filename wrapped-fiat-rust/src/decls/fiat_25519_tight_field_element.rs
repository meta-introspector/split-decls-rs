macro_rules! fiat_25519_tight_field_element {
    () => {
        # [doc = " The type fiat_25519_tight_field_element is a field element with tight bounds. "] # [doc = " Bounds: [[0x0 ~> 0x8000000000000], [0x0 ~> 0x8000000000000], [0x0 ~> 0x8000000000000], [0x0 ~> 0x8000000000000], [0x0 ~> 0x8000000000000]] "] # [derive (Clone , Copy)] pub struct fiat_25519_tight_field_element (pub [u64 ; 5]) ;
    };
}

fiat_25519_tight_field_element!();