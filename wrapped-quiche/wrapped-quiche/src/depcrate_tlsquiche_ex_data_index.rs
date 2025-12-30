// Generated macro for QUICHE_EX_DATA_INDEX (static)
macro_rules! Depcrate_tlsQUICHE_EX_DATA_INDEX {
() => {
// Module: crate::tls
// Provides: {"QUICHE_EX_DATA_INDEX"}
// Dependencies: {}
# [doc = " BoringSSL ex_data index for quiche connections."] pub static QUICHE_EX_DATA_INDEX : LazyLock < c_int > = LazyLock :: new (| | unsafe { SSL_get_ex_new_index (0 , ptr :: null () , ptr :: null () , ptr :: null () , ptr :: null ()) }) ;
};
}
