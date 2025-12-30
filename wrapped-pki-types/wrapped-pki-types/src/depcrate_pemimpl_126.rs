// Generated macro for impl_126 (impl)
macro_rules! Depcrate_pemimpl_126 {
() => {
// Module: crate::pem
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'a , T : PemObject > SliceIter < 'a , T > { # [doc = " Create a new iterator."] pub fn new (current : & 'a [u8]) -> Self { Self { current , _ty : PhantomData , b64_buf : Vec :: with_capacity (1024) , } } # [doc = " Extract and decode the next supported PEM section from `input`"] # [doc = ""] # [doc = " - `Ok(None)` is returned if there is no PEM section to read from `input`"] # [doc = " - Syntax errors and decoding errors produce a `Err(...)`"] # [doc = " - Otherwise each decoded section is returned with a `Ok(Some((..., remainder)))` where"] # [doc = "   `remainder` is the part of the `input` that follows the returned section"] fn read_section (& mut self) -> Result < Option < (SectionKind , Vec < u8 >) > , Error > { self . b64_buf . clear () ; let mut section = None ; loop { let next_line = if let Some (index) = self . current . iter () . position (| byte | * byte == b'\n' || * byte == b'\r') { let (line , newline_plus_remainder) = self . current . split_at (index) ; self . current = & newline_plus_remainder [1 ..] ; Some (line) } else if ! self . current . is_empty () { let next_line = self . current ; self . current = & [] ; Some (next_line) } else { None } ; match read (next_line , & mut section , & mut self . b64_buf) ? { ControlFlow :: Continue (()) => continue , ControlFlow :: Break (item) => return Ok (item) , } } } # [doc = " Returns the rest of the unparsed data."] # [doc = ""] # [doc = " This is the slice immediately following the most"] # [doc = " recently returned item from `next()`."] # [doc (hidden)] pub fn remainder (& self) -> & 'a [u8] { self . current } }
};
}
