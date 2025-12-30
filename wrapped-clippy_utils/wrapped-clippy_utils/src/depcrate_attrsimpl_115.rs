// Generated macro for impl_115 (impl)
macro_rules! Depcrate_attrsimpl_115 {
() => {
// Module: crate::attrs
// Provides: {"impl_115"}
// Dependencies: {}
impl LimitStack { # [must_use] pub fn new (limit : u64) -> Self { Self { stack : vec ! [limit] } } pub fn limit (& self) -> u64 { * self . stack . last () . expect ("there should always be a value in the stack") } pub fn push_attrs (& mut self , sess : & Session , attrs : & [impl AttributeExt] , name : Symbol) { let stack = & mut self . stack ; parse_attrs (sess , attrs , name , | val | stack . push (val)) ; } pub fn pop_attrs (& mut self , sess : & Session , attrs : & [impl AttributeExt] , name : Symbol) { let stack = & mut self . stack ; parse_attrs (sess , attrs , name , | val | assert_eq ! (stack . pop () , Some (val))) ; } }
};
}
