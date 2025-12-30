// Generated macro for macro_1008 (macro)
macro_rules! Depcrate_ssl_connectormacro_1008 {
() => {
// Module: crate::ssl::connector
// Provides: {"macro_1008"}
// Dependencies: {}
cfg_if ! { if # [cfg (ossl110)] { # [allow (clippy :: unnecessary_wraps)] fn setup_curves (_ : & mut SslContextBuilder) -> Result < () , ErrorStack > { Ok (()) } } else if # [cfg (any (ossl102 , libressl))] { fn setup_curves (ctx : & mut SslContextBuilder) -> Result < () , ErrorStack > { ctx . set_ecdh_auto (true) } } else { fn setup_curves (ctx : & mut SslContextBuilder) -> Result < () , ErrorStack > { use crate :: ec :: EcKey ; use crate :: nid :: Nid ; let curve = EcKey :: from_curve_name (Nid :: X9_62_PRIME256V1) ?; ctx . set_tmp_ecdh (& curve) } } }
};
}
