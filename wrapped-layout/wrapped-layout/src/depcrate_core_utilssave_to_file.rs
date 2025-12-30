// Generated macro for save_to_file (function)
macro_rules! Depcrate_core_utilssave_to_file {
() => {
// Module: crate::core::utils
// Provides: {"save_to_file"}
// Dependencies: {}
pub fn save_to_file (filename : & str , content : & str) -> Result < () , Error > { let f = File :: create (filename) ? ; let _ = write ! (& f , "{}" , content) ; # [cfg (feature = "log")] log :: info ! ("Wrote {}" , filename) ; Result :: Ok (()) }
};
}
