macro_rules! fiat_p521_loose_field_element {
    () => {
        # [doc = " The type fiat_p521_loose_field_element is a field element with loose bounds. "] # [doc = " Bounds: [[0x0 ~> 0xc00000000000000], [0x0 ~> 0xc00000000000000], [0x0 ~> 0xc00000000000000], [0x0 ~> 0xc00000000000000], [0x0 ~> 0xc00000000000000], [0x0 ~> 0xc00000000000000], [0x0 ~> 0xc00000000000000], [0x0 ~> 0xc00000000000000], [0x0 ~> 0x600000000000000]] "] # [derive (Clone , Copy)] pub struct fiat_p521_loose_field_element (pub [u64 ; 9]) ;
    };
}

fiat_p521_loose_field_element!()