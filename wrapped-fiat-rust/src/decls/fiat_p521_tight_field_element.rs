macro_rules! fiat_p521_tight_field_element {
    () => {
        # [doc = " The type fiat_p521_tight_field_element is a field element with tight bounds. "] # [doc = " Bounds: [[0x0 ~> 0x400000000000000], [0x0 ~> 0x400000000000000], [0x0 ~> 0x400000000000000], [0x0 ~> 0x400000000000000], [0x0 ~> 0x400000000000000], [0x0 ~> 0x400000000000000], [0x0 ~> 0x400000000000000], [0x0 ~> 0x400000000000000], [0x0 ~> 0x200000000000000]] "] # [derive (Clone , Copy)] pub struct fiat_p521_tight_field_element (pub [u64 ; 9]) ;
    };
}

fiat_p521_tight_field_element!()