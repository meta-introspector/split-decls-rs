// Generated macro for impl_136 (impl)
macro_rules! Depcrateimpl_136 {
() => {
// Module: crate
// Provides: {"impl_136"}
// Dependencies: {}
# [cfg (feature = "tzdb")] impl Get { fn from_id (id : & str) -> Result < Get , String > { let (id , data) = jiff_tzdb :: get (id) . ok_or_else (| | { format ! ("could not find time zone `{id}` in bundled tzdb") }) ? ; let id = id . to_string () ; let tzif = TzifOwned :: parse (Some (id . clone ()) , & data) . map_err (| e | { format ! ("failed to parse TZif data from bundled `{id}`: {e}") }) ? ; Ok (Get { tzif }) } fn quote (& self) -> proc_macro2 :: TokenStream { self . tzif . quote () } }
};
}
