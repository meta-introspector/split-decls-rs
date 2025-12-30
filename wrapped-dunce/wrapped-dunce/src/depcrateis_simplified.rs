// Generated macro for is_simplified (function)
macro_rules! Depcrateis_simplified {
() => {
// Module: crate
// Provides: {"is_simplified"}
// Dependencies: {}
# [doc = " Returns `true` if the path is relative or starts with a disk prefix (like \"C:\")."] # [cfg_attr (not (windows) , allow (unused))] # [must_use] pub fn is_simplified (path : & Path) -> bool { # [cfg (windows)] if let Some (Component :: Prefix (prefix)) = path . components () . next () { return matches ! (prefix . kind () , Prefix :: Disk (..)) } true }
};
}
