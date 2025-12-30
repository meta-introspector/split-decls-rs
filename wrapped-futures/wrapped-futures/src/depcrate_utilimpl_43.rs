// Generated macro for impl_43 (impl)
macro_rules! Depcrate_utilimpl_43 {
() => {
// Module: crate::util
// Provides: {"impl_43"}
// Dependencies: {}
impl < T : Future > Collapsed < T > { pub fn poll (& mut self , task : & mut Task) -> Poll < T :: Item , T :: Error > { match * self { Collapsed :: Start (ref mut a) => a . poll (task) , Collapsed :: Tail (ref mut a) => a . poll (task) , } } pub fn schedule (& mut self , task : & mut Task) { match * self { Collapsed :: Start (ref mut a) => a . schedule (task) , Collapsed :: Tail (ref mut a) => a . schedule (task) , } } pub fn collapse (& mut self) { let a = match * self { Collapsed :: Start (ref mut a) => { match a . tailcall () { Some (a) => a , None => return , } } Collapsed :: Tail (ref mut a) => { if let Some (b) = a . tailcall () { * a = b ; } return } } ; * self = Collapsed :: Tail (a) ; } }
};
}
