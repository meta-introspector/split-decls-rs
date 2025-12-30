// Generated macro for Request (struct)
macro_rules! Depcrate_registryRequest {
() => {
// Module: crate::registry
// Provides: {"Request"}
// Dependencies: {}
# [doc = " Request to the test http server"] # [derive (Clone)] pub struct Request { pub url : Url , pub method : String , pub body : Option < Vec < u8 > > , pub authorization : Option < String > , pub if_modified_since : Option < String > , pub if_none_match : Option < String > , }
};
}
