// Generated macro for impl_1307 (impl)
macro_rules! Depcrate_readimpl_1307 {
() => {
// Module: crate::read
// Provides: {"impl_1307"}
// Dependencies: {}
impl < 'data > ObjectMapFile < 'data > { # [cfg (feature = "macho")] fn new (path : & 'data [u8] , member : Option < & 'data [u8] >) -> Self { ObjectMapFile { path , member } } # [doc = " Get the path to the file containing the object."] # [inline] pub fn path (& self) -> & 'data [u8] { self . path } # [doc = " If the file is an archive, get the name of the member containing the object."] # [inline] pub fn member (& self) -> Option < & 'data [u8] > { self . member } }
};
}
