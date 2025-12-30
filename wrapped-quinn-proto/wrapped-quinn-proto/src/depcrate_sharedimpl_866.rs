// Generated macro for impl_866 (impl)
macro_rules! Depcrate_sharedimpl_866 {
() => {
// Module: crate::shared
// Provides: {"impl_866"}
// Dependencies: {}
impl ConnectionId { # [doc = " Construct cid from byte array"] pub fn new (bytes : & [u8]) -> Self { debug_assert ! (bytes . len () <= MAX_CID_SIZE) ; let mut res = Self { len : bytes . len () as u8 , bytes : [0 ; MAX_CID_SIZE] , } ; res . bytes [.. bytes . len ()] . copy_from_slice (bytes) ; res } # [doc = " Constructs cid by reading `len` bytes from a `Buf`"] # [doc = ""] # [doc = " Callers need to assure that `buf.remaining() >= len`"] pub fn from_buf (buf : & mut (impl Buf + ? Sized) , len : usize) -> Self { debug_assert ! (len <= MAX_CID_SIZE) ; let mut res = Self { len : len as u8 , bytes : [0 ; MAX_CID_SIZE] , } ; buf . copy_to_slice (& mut res [.. len]) ; res } # [doc = " Decode from long header format"] pub (crate) fn decode_long (buf : & mut impl Buf) -> Option < Self > { let len = buf . get :: < u8 > () . ok () ? as usize ; match len > MAX_CID_SIZE || buf . remaining () < len { false => Some (Self :: from_buf (buf , len)) , true => None , } } # [doc = " Encode in long header format"] pub (crate) fn encode_long (& self , buf : & mut impl BufMut) { buf . put_u8 (self . len () as u8) ; buf . put_slice (self) ; } }
};
}
