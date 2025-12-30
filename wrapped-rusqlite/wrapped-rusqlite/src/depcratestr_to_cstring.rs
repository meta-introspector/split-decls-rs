// Generated macro for str_to_cstring (function)
macro_rules! Depcratestr_to_cstring {
() => {
// Module: crate
// Provides: {"str_to_cstring"}
// Dependencies: {}
# [cfg (any (feature = "functions" , feature = "vtab" , test))] fn str_to_cstring (s : & str) -> Result < util :: SmallCString > { Ok (util :: SmallCString :: new (s) ?) }
};
}
