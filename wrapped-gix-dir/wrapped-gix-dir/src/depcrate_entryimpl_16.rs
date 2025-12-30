// Generated macro for impl_16 (impl)
macro_rules! Depcrate_entryimpl_16 {
() => {
// Module: crate::entry
// Provides: {"impl_16"}
// Dependencies: {}
# [doc = " Conversion"] impl Entry { # [doc = " Obtain an [`EntryRef`] from this instance."] pub fn to_ref (& self) -> EntryRef < '_ > { EntryRef { rela_path : Cow :: Borrowed (self . rela_path . as_ref ()) , status : self . status , property : self . property , disk_kind : self . disk_kind , index_kind : self . index_kind , pathspec_match : self . pathspec_match , } } }
};
}
