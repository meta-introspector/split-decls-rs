// Generated macro for impl_472 (impl)
macro_rules! Depcrate_read_arangesimpl_472 {
() => {
// Module: crate::read::aranges
// Provides: {"impl_472"}
// Dependencies: {}
impl ArangeEntry { # [doc = " Parse a single arange. Return `None` for the null arange, `Some` for an actual arange."] fn parse < R : Reader > (input : & mut R , encoding : Encoding) -> Result < Option < Self > > { let address_size = encoding . address_size ; let tuple_length = R :: Offset :: from_u8 (2 * address_size) ; if tuple_length > input . len () { input . empty () ; return Ok (None) ; } let begin = input . read_address (address_size) ? ; let length = input . read_address (address_size) ? ; let range = Range { begin , end : 0 } ; match (begin , length) { (0 , 0) => Self :: parse (input , encoding) , _ => Ok (Some (ArangeEntry { range , length })) , } } # [doc = " Return the beginning address of this arange."] # [inline] pub fn address (& self) -> u64 { self . range . begin } # [doc = " Return the length of this arange."] # [inline] pub fn length (& self) -> u64 { self . length } # [doc = " Return the range."] # [inline] pub fn range (& self) -> Range { self . range } }
};
}
