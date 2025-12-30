// Generated macro for target_feature (function)
macro_rules! Depcrate_wasm_conventionstarget_feature {
() => {
// Module: crate::wasm_conventions
// Provides: {"target_feature"}
// Dependencies: {}
pub fn target_feature (module : & Module , feature : & str) -> Result < bool > { anyhow :: ensure ! (feature . len () <= 100_000 , "feature name too long") ; let section = module . customs . iter () . find (| (_ , custom) | custom . name () == "target_features") ; if let Some ((_ , section)) = section { let section : & RawCustomSection = section . as_any () . downcast_ref () . context ("failed to read section") ? ; let mut reader = BinaryReader :: new (& section . data , 0) ; let count = reader . read_var_u32 () ? ; for _ in 0 .. count { let prefix = reader . read_u8 () ? ; let length = reader . read_var_u32 () ? ; let this_feature = reader . read_bytes (length as usize) ? ; if this_feature == feature . as_bytes () { if prefix == b'-' { return Ok (false) ; } return Ok (true) ; } } Ok (false) } else { Ok (false) } }
};
}
