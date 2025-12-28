macro_rules! deps {
    () => {
        ThreadSafeRepository!();
        Proxy!();
        Repository!();
        Cache!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl From < & crate :: ThreadSafeRepository > for crate :: Repository { fn from (repo : & crate :: ThreadSafeRepository) -> Self { crate :: Repository :: from_refs_and_objects (repo . refs . clone () , gix_odb :: memory :: Proxy :: from (gix_odb :: Cache :: from (repo . objects . to_handle ())) . with_write_passthrough () , repo . work_tree . clone () , repo . common_dir . clone () , repo . config . clone () , repo . linked_worktree_options . clone () , # [cfg (feature = "index")] repo . index . clone () , repo . shallow_commits . clone () , # [cfg (feature = "attributes")] repo . modules . clone () ,) } }
    };
}

impl_332!();