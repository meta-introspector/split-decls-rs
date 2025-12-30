// Generated macro for Name (enum)
macro_rules! Depcrate_hpack_headerName {
() => {
// Module: crate::hpack::header
// Provides: {"Name"}
// Dependencies: {}
# [doc = " The header field name"] # [derive (Debug , Clone , Eq , PartialEq , Hash)] pub enum Name < 'a > { Field (& 'a HeaderName) , Authority , Method , Scheme , Path , Protocol , Status , }
};
}
