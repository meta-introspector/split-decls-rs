// Generated macro for impl_820 (impl)
macro_rules! Depcrate_h3_qpack_encoderimpl_820 {
() => {
// Module: crate::h3::qpack::encoder
// Provides: {"impl_820"}
// Dependencies: {}
impl Encoder { # [doc = " Creates a new QPACK encoder."] pub fn new () -> Encoder { Encoder :: default () } # [doc = " Encodes a list of headers into a QPACK header block."] pub fn encode < T : NameValue > (& mut self , headers : & [T] , out : & mut [u8] ,) -> Result < usize > { let mut b = octets :: OctetsMut :: with_slice (out) ; encode_int (0 , 0 , 8 , & mut b) ? ; encode_int (0 , 0 , 7 , & mut b) ? ; for h in headers { match lookup_static (h) { Some ((idx , true)) => { const STATIC : u8 = 0x40 ; encode_int (idx , INDEXED | STATIC , 6 , & mut b) ? ; } , Some ((idx , false)) => { const STATIC : u8 = 0x10 ; encode_int (idx , LITERAL_WITH_NAME_REF | STATIC , 4 , & mut b) ? ; encode_str :: < false > (h . value () , 0 , 7 , & mut b) ? ; } , None => { encode_str :: < true > (h . name () , LITERAL , 3 , & mut b) ? ; encode_str :: < false > (h . value () , 0 , 7 , & mut b) ? ; } , } ; } Ok (b . off ()) } }
};
}
