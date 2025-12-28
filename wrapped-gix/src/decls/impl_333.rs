macro_rules! deps {
    () => {
        Proxy!();
        Cache!();
        Repository!();
        ThreadSafeRepository!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl From < crate :: ThreadSafeRepository > for crate :: Repository { fn from (repo : crate :: ThreadSafeRepository) -> Self { crate :: Repository :: from_refs_and_objects (repo . refs , gix_odb :: memory :: Proxy :: from (gix_odb :: Cache :: from (repo . objects . to_handle ())) . with_write_passthrough () , repo . work_tree , repo . common_dir , repo . config , repo . linked_worktree_options , # [cfg (feature = "index")] repo . index , repo . shallow_commits , # [cfg (feature = "attributes")] repo . modules . clone () ,) } }
    };
}

impl_333!();