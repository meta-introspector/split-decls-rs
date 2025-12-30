// Generated macro for impl_142 (impl)
macro_rules! Depcrateimpl_142 {
() => {
// Module: crate
// Provides: {"impl_142"}
// Dependencies: {}
impl PartialErrorBuilder { fn push (& mut self , err : Error) { self . 0 . push (err) ; } fn push_ignore_io (& mut self , err : Error) { if ! err . is_io () { self . push (err) ; } } fn maybe_push (& mut self , err : Option < Error >) { if let Some (err) = err { self . push (err) ; } } fn maybe_push_ignore_io (& mut self , err : Option < Error >) { if let Some (err) = err { self . push_ignore_io (err) ; } } fn into_error_option (mut self) -> Option < Error > { if self . 0 . is_empty () { None } else if self . 0 . len () == 1 { Some (self . 0 . pop () . unwrap ()) } else { Some (Error :: Partial (self . 0)) } } }
};
}
