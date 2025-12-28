macro_rules! fiat_poly1305_tight_field_element {
    () => {
        # [doc = " The type fiat_poly1305_tight_field_element is a field element with tight bounds. "] # [doc = " Bounds: [[0x0 ~> 0x100000000000], [0x0 ~> 0x80000000000], [0x0 ~> 0x80000000000]] "] # [derive (Clone , Copy)] pub struct fiat_poly1305_tight_field_element (pub [u64 ; 3]) ;
    };
}

fiat_poly1305_tight_field_element!();