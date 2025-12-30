// Generated macro for as_str (function)
macro_rules! Depcrate_repository_statusas_str {
() => {
// Module: crate::repository::status
// Provides: {"as_str"}
// Dependencies: {}
fn as_str (c : Conflict) -> & 'static str { match c { Conflict :: BothDeleted => "DD" , Conflict :: AddedByUs => "AU" , Conflict :: DeletedByThem => "UD" , Conflict :: AddedByThem => "UA" , Conflict :: DeletedByUs => "DU" , Conflict :: BothAdded => "AA" , Conflict :: BothModified => "UU" , } }
};
}
