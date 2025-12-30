// Generated macro for macro_55 (macro)
macro_rules! Depcrate_aesmacro_55 {
() => {
// Module: crate::aes
// Provides: {"macro_55"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (boringssl , awslc))] { type AesBitType = c_uint ; type AesSizeType = usize ; } else { type AesBitType = c_int ; type AesSizeType = c_uint ; } }
};
}
