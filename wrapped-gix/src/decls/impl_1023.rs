macro_rules! deps {
    () => {
        Error!();
        Options!();
        Outcome!();
        Default!();
        IndexPersistedOrInMemory!();
        ApplyChange!();
    };
}

macro_rules! impl_1023 {
    () => {
        deps!();
        impl Outcome { # [doc = " Returns `true` if the index has received currently unapplied changes that *should* be written back."] # [doc = ""] # [doc = " If they are not written back, subsequent `status` operations will take longer to complete, whereas the"] # [doc = " additional work can be prevented by writing the changes back to the index."] pub fn has_changes (& self) -> bool { self . changes . as_ref () . is_some_and (| changes | ! changes . is_empty ()) } # [doc = " Write the changes if there are any back to the index file."] # [doc = " This can only be done once as the changes are consumed in the process, if there were any."] pub fn write_changes (& mut self) -> Option < Result < () , gix_index :: file :: write :: Error > > { let _span = gix_features :: trace :: coarse ! ("gix::status::index_worktree::Outcome::write_changes()") ; let changes = self . changes . take () ? ; let mut index = match & self . worktree_index { IndexPersistedOrInMemory :: Persisted (persisted) => (* * * persisted) . clone () , IndexPersistedOrInMemory :: InMemory (index) => index . clone () , } ; let entries = index . entries_mut () ; for (entry_index , change) in changes { let entry = & mut entries [entry_index] ; match change { ApplyChange :: SetSizeToZero => { entry . stat . size = 0 ; } ApplyChange :: NewStat (new_stat) => { entry . stat = new_stat ; } } } Some (index . write (crate :: index :: write :: Options { extensions : Default :: default () , skip_hash : self . skip_hash , })) } }
    };
}

impl_1023!();