macro_rules! fiat_p448_loose_field_element {
    () => {
        # [doc = " The type fiat_p448_loose_field_element is a field element with loose bounds. "] # [doc = " Bounds: [[0x0 ~> 0x300000000000000], [0x0 ~> 0x300000000000000], [0x0 ~> 0x300000000000000], [0x0 ~> 0x300000000000000], [0x0 ~> 0x300000000000000], [0x0 ~> 0x300000000000000], [0x0 ~> 0x300000000000000], [0x0 ~> 0x300000000000000]] "] # [derive (Clone , Copy)] pub struct fiat_p448_loose_field_element (pub [u64 ; 8]) ;
    };
}

fiat_p448_loose_field_element!();