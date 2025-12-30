// Generated macro for remove_dep (function)
macro_rules! Depcrate_helpersremove_dep {
() => {
// Module: crate::helpers
// Provides: {"remove_dep"}
// Dependencies: {}
pub fn remove_dep (sum : & Summary , ind : usize) -> Summary { let mut deps = sum . dependencies () . to_vec () ; deps . remove (ind) ; Summary :: new (sum . package_id () , deps , & BTreeMap :: new () , sum . links () , None) . unwrap () }
};
}
