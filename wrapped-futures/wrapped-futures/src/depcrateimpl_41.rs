// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl From < Promise > for JsFuture { fn from (js : Promise) -> JsFuture { let state = Rc :: new (RefCell :: new (Inner { result : None , task : None , callbacks : None , })) ; fn finish (state : & RefCell < Inner > , val : Result < JsValue , JsValue >) { let task = { let mut state = state . borrow_mut () ; debug_assert ! (state . callbacks . is_some ()) ; debug_assert ! (state . result . is_none ()) ; drop (state . callbacks . take ()) ; state . result = Some (val) ; state . task . take () } ; if let Some (task) = task { task . wake () } } let resolve = { let state = state . clone () ; Closure :: once (move | val | finish (& state , Ok (val))) } ; let reject = { let state = state . clone () ; Closure :: once (move | val | finish (& state , Err (val))) } ; let _ = js . then2 (& resolve , & reject) ; state . borrow_mut () . callbacks = Some ((resolve , reject)) ; JsFuture { inner : state } } }
};
}
