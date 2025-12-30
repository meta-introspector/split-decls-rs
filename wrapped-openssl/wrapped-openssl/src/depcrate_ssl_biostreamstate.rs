// Generated macro for StreamState (struct)
macro_rules! Depcrate_ssl_bioStreamState {
() => {
// Module: crate::ssl::bio
// Provides: {"StreamState"}
// Dependencies: {}
pub struct StreamState < S > { pub stream : S , pub error : Option < io :: Error > , pub panic : Option < Box < dyn Any + Send > > , pub dtls_mtu_size : c_long , }
};
}
