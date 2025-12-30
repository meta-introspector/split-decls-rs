// Generated macro for macro_26 (macro)
macro_rules! Depcratemacro_26 {
() => {
// Module: crate
// Provides: {"macro_26"}
// Dependencies: {}
if_conversions ! { fn cow_os_str_into_path (string : Cow <'_ , OsStr >) -> Cow <'_ , Path > { match string { Cow :: Borrowed (string) => Cow :: Borrowed (Path :: new (string)) , Cow :: Owned (string) => Cow :: Owned (string . into ()) , } } }
};
}
