// Generated macro for impl_25 (impl)
macro_rules! Depcrate_boltimpl_25 {
() => {
// Module: crate::bolt
// Provides: {"impl_25"}
// Dependencies: {}
impl BackedUpFile { pub fn new (file : & Utf8Path) -> anyhow :: Result < Self > { let temp_path = tempfile :: NamedTempFile :: new () ? . into_temp_path () ; copy_file (file , & temp_path) ? ; Ok (Self { backup : temp_path , original : file . to_path_buf () }) } }
};
}
