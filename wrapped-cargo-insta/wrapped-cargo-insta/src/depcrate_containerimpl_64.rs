// Generated macro for impl_64 (impl)
macro_rules! Depcrate_containerimpl_64 {
() => {
// Module: crate::container
// Provides: {"impl_64"}
// Dependencies: {}
impl PendingSnapshot { pub (crate) fn summary (& self) -> String { use std :: fmt :: Write ; let mut rv = String :: new () ; if let Some (source) = self . new . metadata () . source () { write ! (& mut rv , "{source}") . unwrap () ; } if let Some (line) = self . line { write ! (& mut rv , ":{line}") . unwrap () ; } if let Some (name) = self . new . snapshot_name () { write ! (& mut rv , " ({name})") . unwrap () ; } rv } }
};
}
