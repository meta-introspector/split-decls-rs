// Generated macro for commit_type_by_parents (function)
macro_rules! Depcrate_reference_logcommit_type_by_parents {
() => {
// Module: crate::reference::log
// Provides: {"commit_type_by_parents"}
// Dependencies: {}
pub (crate) fn commit_type_by_parents (count : usize) -> Option < & 'static str > { Some (match count { 0 => "initial" , 1 => return None , _two_or_more => "merge" , }) }
};
}
