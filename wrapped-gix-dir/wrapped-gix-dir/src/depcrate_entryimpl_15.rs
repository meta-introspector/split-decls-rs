// Generated macro for impl_15 (impl)
macro_rules! Depcrate_entryimpl_15 {
() => {
// Module: crate::entry
// Provides: {"impl_15"}
// Dependencies: {}
# [doc = " Conversion"] impl EntryRef < '_ > { # [doc = " Strip the lifetime to obtain a fully owned copy."] pub fn to_owned (& self) -> Entry { Entry { rela_path : self . rela_path . clone () . into_owned () , status : self . status , property : self . property , disk_kind : self . disk_kind , index_kind : self . index_kind , pathspec_match : self . pathspec_match , } } # [doc = " Turn this instance into a fully owned copy."] pub fn into_owned (self) -> Entry { Entry { rela_path : self . rela_path . into_owned () , status : self . status , property : self . property , disk_kind : self . disk_kind , index_kind : self . index_kind , pathspec_match : self . pathspec_match , } } }
};
}
