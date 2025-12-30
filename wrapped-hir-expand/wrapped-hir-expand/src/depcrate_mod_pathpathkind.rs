// Generated macro for PathKind (enum)
macro_rules! Depcrate_mod_pathPathKind {
() => {
// Module: crate::mod_path
// Provides: {"PathKind"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum PathKind { Plain , # [doc = " `self::` is `Super(0)`"] Super (u8) , Crate , # [doc = " Absolute path (::foo)"] Abs , # [doc = " `$crate` from macro expansion"] DollarCrate (Crate) , }
};
}
