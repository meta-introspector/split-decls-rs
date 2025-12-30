// Generated macro for ProcMacro (enum)
macro_rules! Depcrate_bridge_clientProcMacro {
() => {
// Module: crate::bridge::client
// Provides: {"ProcMacro"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone)] pub enum ProcMacro { CustomDerive { trait_name : & 'static str , attributes : & 'static [& 'static str] , client : Client < crate :: TokenStream , crate :: TokenStream > , } , Attr { name : & 'static str , client : Client < (crate :: TokenStream , crate :: TokenStream) , crate :: TokenStream > , } , Bang { name : & 'static str , client : Client < crate :: TokenStream , crate :: TokenStream > , } , }
};
}
