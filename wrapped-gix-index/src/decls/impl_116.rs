macro_rules! deps {
    () => {
        FsMonitor!();
        Link!();
        UntrackedCache!();
        State!();
        Extensions!();
        Tree!();
        Paths!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        # [doc = " Extensions"] impl State { # [doc = " Access the `tree` extension."] pub fn tree (& self) -> Option < & extension :: Tree > { self . tree . as_ref () } # [doc = " Remove the `tree` extension."] pub fn remove_tree (& mut self) -> Option < extension :: Tree > { self . tree . take () } # [doc = " Access the `link` extension."] pub fn link (& self) -> Option < & extension :: Link > { self . link . as_ref () } # [doc = " Obtain the resolve-undo extension."] pub fn resolve_undo (& self) -> Option < & extension :: resolve_undo :: Paths > { self . resolve_undo . as_ref () } # [doc = " Remove the resolve-undo extension."] pub fn remove_resolve_undo (& mut self) -> Option < extension :: resolve_undo :: Paths > { self . resolve_undo . take () } # [doc = " Obtain the untracked extension."] pub fn untracked (& self) -> Option < & extension :: UntrackedCache > { self . untracked . as_ref () } # [doc = " Obtain the fsmonitor extension."] pub fn fs_monitor (& self) -> Option < & extension :: FsMonitor > { self . fs_monitor . as_ref () } # [doc = " Return `true` if the end-of-index extension was present when decoding this index."] pub fn had_end_of_index_marker (& self) -> bool { self . end_of_index_at_decode_time } # [doc = " Return `true` if the offset-table extension was present when decoding this index."] pub fn had_offset_table (& self) -> bool { self . offset_table_at_decode_time } }
    };
}

impl_116!();