// Generated macro for Export (struct)
macro_rules! Depcrate_readExport {
() => {
// Module: crate::read
// Provides: {"Export"}
// Dependencies: {}
# [doc = " An exported symbol."] # [doc = ""] # [doc = " Returned by [`Object::exports`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Export < 'data > { name : ByteString < 'data > , address : u64 , }
};
}
