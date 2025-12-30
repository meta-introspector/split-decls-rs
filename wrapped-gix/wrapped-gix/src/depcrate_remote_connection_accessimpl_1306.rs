// Generated macro for impl_1306 (impl)
macro_rules! Depcrate_remote_connection_accessimpl_1306 {
() => {
// Module: crate::remote::connection::access
// Provides: {"impl_1306"}
// Dependencies: {}
# [doc = " Mutation"] impl < 'a , T > Connection < 'a , '_ , T > where T : Transport , { # [doc = " Like [`with_credentials()`](Self::with_credentials()), but without consuming the connection."] pub fn set_credentials (& mut self , helper : impl FnMut (gix_credentials :: helper :: Action) -> gix_credentials :: protocol :: Result + 'a ,) -> & mut Self { self . authenticate = Some (Box :: new (helper)) ; self } # [doc = " Like [`with_transport_options()`](Self::with_transport_options()), but without consuming the connection."] pub fn set_transport_options (& mut self , config : Box < dyn std :: any :: Any >) -> & mut Self { self . transport_options = Some (config) ; self } }
};
}
