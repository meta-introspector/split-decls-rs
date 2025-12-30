// Generated macro for NZCV (struct)
macro_rules! Depcrate_isa_aarch64_inst_immsNZCV {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"NZCV"}
// Dependencies: {}
# [doc = " An immediate that represents the NZCV flags."] # [derive (Clone , Copy , Debug)] pub struct NZCV { # [doc = " The negative condition flag."] n : bool , # [doc = " The zero condition flag."] z : bool , # [doc = " The carry condition flag."] c : bool , # [doc = " The overflow condition flag."] v : bool , }
};
}
