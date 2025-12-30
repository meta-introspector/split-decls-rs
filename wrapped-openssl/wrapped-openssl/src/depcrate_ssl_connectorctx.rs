// Generated macro for ctx (function)
macro_rules! Depcrate_ssl_connectorctx {
() => {
// Module: crate::ssl::connector
// Provides: {"ctx"}
// Dependencies: {}
# [allow (clippy :: inconsistent_digit_grouping , clippy :: unusual_byte_groupings)] fn ctx (method : SslMethod) -> Result < SslContextBuilder , ErrorStack > { let mut ctx = SslContextBuilder :: new (method) ? ; cfg_if ! { if # [cfg (not (any (boringssl , awslc)))] { let mut opts = SslOptions :: ALL | SslOptions :: NO_COMPRESSION | SslOptions :: NO_SSLV2 | SslOptions :: NO_SSLV3 | SslOptions :: SINGLE_DH_USE | SslOptions :: SINGLE_ECDH_USE ; opts &= ! SslOptions :: DONT_INSERT_EMPTY_FRAGMENTS ; ctx . set_options (opts) ; } } let mut mode = SslMode :: AUTO_RETRY | SslMode :: ACCEPT_MOVING_WRITE_BUFFER | SslMode :: ENABLE_PARTIAL_WRITE ; if version :: number () >= 0x1_00_01_08_0 { mode |= SslMode :: RELEASE_BUFFERS ; } ctx . set_mode (mode) ; Ok (ctx) }
};
}
