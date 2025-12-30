// Generated macro for impl_23 (impl)
macro_rules! Depcrate_hdrimpl_23 {
() => {
// Module: crate::hdr
// Provides: {"impl_23"}
// Dependencies: {}
impl From < Header > for Title { # [inline (always)] fn from (header : Header) -> Self { let int = | i : u64 | match i { x if x <= 23 => Minor :: This (i as u8) , x if x <= u8 :: MAX as u64 => Minor :: Next1 ([i as u8]) , x if x <= u16 :: MAX as u64 => Minor :: Next2 ((i as u16) . to_be_bytes ()) , x if x <= u32 :: MAX as u64 => Minor :: Next4 ((i as u32) . to_be_bytes ()) , x => Minor :: Next8 (x . to_be_bytes ()) , } ; let len = | l : Option < usize > | l . map (| x | int (x as u64)) . unwrap_or (Minor :: More) ; match header { Header :: Positive (x) => Title (Major :: Positive , int (x)) , Header :: Negative (x) => Title (Major :: Negative , int (x)) , Header :: Bytes (x) => Title (Major :: Bytes , len (x)) , Header :: Text (x) => Title (Major :: Text , len (x)) , Header :: Array (x) => Title (Major :: Array , len (x)) , Header :: Map (x) => Title (Major :: Map , len (x)) , Header :: Tag (x) => Title (Major :: Tag , int (x)) , Header :: Break => Title (Major :: Other , Minor :: More) , Header :: Simple (x) => match x { x @ 0 ..= 23 => Title (Major :: Other , Minor :: This (x)) , x => Title (Major :: Other , Minor :: Next1 ([x])) , } , Header :: Float (n64) => { # [cfg (feature = "half")] let n16 = f16 :: from_f64 (n64) ; # [cfg (not (feature = "half"))] let n16 = n64 as f16 ; let n32 = n64 as f32 ; Title (Major :: Other , if f64 :: from (n16) . to_bits () == n64 . to_bits () { Minor :: Next2 (n16 . to_be_bytes ()) } else if f64 :: from (n32) . to_bits () == n64 . to_bits () { Minor :: Next4 (n32 . to_be_bytes ()) } else { Minor :: Next8 (n64 . to_be_bytes ()) } ,) } } } }
};
}
