// Generated macro for ReadDataImpl (struct)
macro_rules! Depcrate_index_as_worktree_functionReadDataImpl {
() => {
// Module: crate::index_as_worktree::function
// Provides: {"ReadDataImpl"}
// Dependencies: {}
struct ReadDataImpl < 'a , Find > where Find : gix_object :: Find , { buf : & 'a mut Vec < u8 > , path : & 'a Path , rela_path : & 'a BStr , file_len : u64 , entry : & 'a gix_index :: Entry , filter : & 'a mut gix_filter :: Pipeline , attr_stack : & 'a mut gix_worktree :: Stack , core_symlinks : bool , id : & 'a gix_hash :: oid , objects : Find , worktree_bytes : & 'a AtomicU64 , worktree_reads : & 'a AtomicUsize , odb_bytes : & 'a AtomicU64 , odb_reads : & 'a AtomicUsize , }
};
}
