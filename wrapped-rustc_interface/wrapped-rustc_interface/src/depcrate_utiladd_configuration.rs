// Generated macro for add_configuration (function)
macro_rules! Depcrate_utiladd_configuration {
() => {
// Module: crate::util
// Provides: {"add_configuration"}
// Dependencies: {}
# [doc = " Adds `target_feature = \"...\"` cfgs for a variety of platform"] # [doc = " specific features (SSE, NEON etc.)."] # [doc = ""] # [doc = " This is performed by checking whether a set of permitted features"] # [doc = " is available on the target machine, by querying the codegen backend."] pub (crate) fn add_configuration (cfg : & mut Cfg , sess : & mut Session , codegen_backend : & dyn CodegenBackend ,) { let tf = sym :: target_feature ; let tf_cfg = codegen_backend . target_config (sess) ; sess . unstable_target_features . extend (tf_cfg . unstable_target_features . iter () . copied ()) ; sess . target_features . extend (tf_cfg . target_features . iter () . copied ()) ; cfg . extend (tf_cfg . target_features . into_iter () . map (| feat | (tf , Some (feat)))) ; if tf_cfg . has_reliable_f16 { cfg . insert ((sym :: target_has_reliable_f16 , None)) ; } if tf_cfg . has_reliable_f16_math { cfg . insert ((sym :: target_has_reliable_f16_math , None)) ; } if tf_cfg . has_reliable_f128 { cfg . insert ((sym :: target_has_reliable_f128 , None)) ; } if tf_cfg . has_reliable_f128_math { cfg . insert ((sym :: target_has_reliable_f128_math , None)) ; } if sess . crt_static (None) { cfg . insert ((tf , Some (sym :: crt_dash_static))) ; } }
};
}
