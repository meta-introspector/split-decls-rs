// Generated macro for impl_40 (impl)
macro_rules! Depcrate_extension_iterimpl_40 {
() => {
// Module: crate::extension::iter
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = (extension :: Signature , & 'a [u8]) ; fn next (& mut self) -> Option < Self :: Item > { if self . data . len () < 4 + 4 { return None ; } let (signature , data) = self . data . split_at (4) ; let (size , data) = data . split_at (4) ; self . data = data ; self . consumed += 4 + 4 ; let size = from_be_u32 (size) as usize ; match data . get (.. size) { Some (ext_data) => { self . data = & data [size ..] ; self . consumed += size ; Some ((signature . try_into () . unwrap () , ext_data)) } None => { self . data = & [] ; None } } } }
};
}
