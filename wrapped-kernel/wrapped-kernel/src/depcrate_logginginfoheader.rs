// Generated macro for infoheader (macro)
macro_rules! Depcrate_logginginfoheader {
() => {
// Module: crate::logging
// Provides: {"infoheader"}
// Dependencies: {}
# [cfg_attr (target_arch = "riscv64" , allow (unused))] macro_rules ! infoheader { ($ str : expr) => { { :: log :: info ! ("") ; :: log :: info ! ("{:=^70}" , $ str) ; } } ; }
};
}
