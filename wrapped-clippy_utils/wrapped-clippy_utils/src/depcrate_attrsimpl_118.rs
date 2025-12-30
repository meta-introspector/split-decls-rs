// Generated macro for impl_118 (impl)
macro_rules! Depcrate_attrsimpl_118 {
() => {
// Module: crate::attrs
// Provides: {"impl_118"}
// Dependencies: {}
# [expect (missing_docs , reason = "they're all trivial...")] impl LimitStack { # [must_use] # [doc = " Initialize the stack starting with a default value, which usually comes from configuration"] pub fn new (limit : u64) -> Self { Self { default : limit , stack : vec ! [] , } } pub fn limit (& self) -> u64 { self . stack . last () . copied () . unwrap_or (self . default) } pub fn push_attrs (& mut self , sess : & Session , attrs : & [impl AttributeExt] , name : Symbol) { let stack = & mut self . stack ; parse_attrs (sess , attrs , name , | val | stack . push (val)) ; } pub fn pop_attrs (& mut self , sess : & Session , attrs : & [impl AttributeExt] , name : Symbol) { let stack = & mut self . stack ; parse_attrs (sess , attrs , name , | val | debug_assert_eq ! (stack . pop () , Some (val))) ; } }
};
}
