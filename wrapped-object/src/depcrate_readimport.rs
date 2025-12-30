// Generated macro for Import (struct)
macro_rules! Depcrate_readImport {
() => {
// Module: crate::read
// Provides: {"Import"}
// Dependencies: {}
# [doc = " An imported symbol."] # [doc = ""] # [doc = " Returned by [`Object::imports`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Import < 'data > { library : ByteString < 'data > , name : ByteString < 'data > , }
};
}
