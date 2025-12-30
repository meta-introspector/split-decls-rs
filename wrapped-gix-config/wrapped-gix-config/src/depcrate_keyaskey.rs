// Generated macro for AsKey (trait)
macro_rules! Depcrate_keyAsKey {
() => {
// Module: crate::key
// Provides: {"AsKey"}
// Dependencies: {}
# [doc = " Parse parts of a Git configuration key, like `remote.origin.url` or `core.bare`."] pub trait AsKey { # [doc = " Return a parsed key reference, containing all relevant parts of a key."] # [doc = " For instance, `remote.origin.url` such key would yield access to `(\"remote\", Some(\"origin\"), \"url\")`"] # [doc = " while `user.name` would yield `(\"user\", None, \"name\")`."] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " If there is no valid `KeyRef` representation."] fn as_key (& self) -> KeyRef < '_ > ; # [doc = " Return a parsed key reference, containing all relevant parts of a key."] # [doc = " For instance, `remote.origin.url` such key would yield access to `(\"remote\", Some(\"origin\"), \"url\")`"] # [doc = " while `user.name` would yield `(\"user\", None, \"name\")`."] fn try_as_key (& self) -> Option < KeyRef < '_ > > ; }
};
}
