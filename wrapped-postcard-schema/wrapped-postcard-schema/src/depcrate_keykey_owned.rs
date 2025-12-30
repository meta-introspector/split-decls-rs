// Generated macro for key_owned (module)
macro_rules! Depcrate_keykey_owned {
() => {
// Module: crate::key
// Provides: {"key_owned"}
// Dependencies: {}
# [cfg (feature = "use-std")] mod key_owned { use super :: * ; use crate :: schema :: owned :: OwnedNamedType ; impl Key { # [doc = " Calculate the Key for the given path and [`OwnedNamedType`]"] pub fn for_owned_schema_path (path : & str , nt : & OwnedNamedType) -> Key { Key (hash :: fnv1a64_owned :: hash_ty_path_owned (path , nt)) } } }
};
}
