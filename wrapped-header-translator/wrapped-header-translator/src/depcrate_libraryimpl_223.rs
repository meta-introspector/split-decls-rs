// Generated macro for impl_223 (impl)
macro_rules! Depcrate_libraryimpl_223 {
() => {
// Module: crate::library
// Provides: {"impl_223"}
// Dependencies: {}
impl < 'a > EntryExt < 'a > for toml_edit :: Entry < 'a > { fn implicit_table (self) -> & 'a mut Table { self . or_insert ({ let mut table = Table :: new () ; table . set_implicit (true) ; Item :: Table (table) }) . as_table_mut () . unwrap () } }
};
}
