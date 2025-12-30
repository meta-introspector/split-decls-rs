// Generated macro for impl_1308 (impl)
macro_rules! Depcrate_shims_unix_fsimpl_1308 {
() => {
// Module: crate::shims::unix::fs
// Provides: {"impl_1308"}
// Dependencies: {}
impl DirTable { # [expect (clippy :: arithmetic_side_effects)] fn insert_new (& mut self , read_dir : ReadDir) -> u64 { let id = self . next_id ; self . next_id += 1 ; self . streams . try_insert (id , OpenDir :: new (read_dir)) . unwrap () ; id } }
};
}
