// Generated macro for impl_17 (impl)
macro_rules! Depcrate_blobimpl_17 {
() => {
// Module: crate::blob
// Provides: {"impl_17"}
// Dependencies: {}
impl Blob { # [doc = " Create a new `Blob` from a `&str`, `&[u8]` or `js_sys::ArrayBuffer`."] pub fn new < T > (content : T) -> Blob where T : BlobContents , { Blob :: new_with_options (content , None) } # [doc = " Like `new`, but allows specifying the MIME type (also known as *content type* or *media"] # [doc = " type*) of the `Blob`."] pub fn new_with_options < T > (content : T , mime_type : Option < & str >) -> Blob where T : BlobContents , { let mut properties = web_sys :: BlobPropertyBag :: new () ; if let Some (mime_type) = mime_type { properties . type_ (mime_type) ; } let parts = js_sys :: Array :: of1 (& unsafe { content . into_jsvalue () }) ; let inner = web_sys :: Blob :: new_with_u8_array_sequence_and_options (& parts , & properties) ; Blob :: from (inner . unwrap_throw ()) } pub fn slice (& self , start : u64 , end : u64) -> Self { let start = safe_u64_to_f64 (start) ; let end = safe_u64_to_f64 (end) ; let b : & web_sys :: Blob = self . as_ref () ; Blob :: from (b . slice_with_f64_and_f64 (start , end) . unwrap_throw ()) } # [doc = " The number of bytes in the Blob/File."] pub fn size (& self) -> u64 { safe_f64_to_u64 (self . inner . size ()) } # [doc = " The statically typed MIME type (also known as *content type* or *media type*) of the `File`"] # [doc = " or `Blob`."] # [cfg (feature = "mime")] pub fn mime_type (& self) -> Result < mime :: Mime , mime :: FromStrError > { self . raw_mime_type () . parse () } # [doc = " The raw MIME type (also known as *content type* or *media type*) of the `File` or"] # [doc = " `Blob`."] pub fn raw_mime_type (& self) -> String { self . inner . type_ () } }
};
}
