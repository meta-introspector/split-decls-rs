// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl SourceWithState { fn change_namespace (& mut self , target : & [String]) { let mut same = 0 ; for (a , b) in self . namespace . iter () . zip (target . iter ()) { if a == b { same += 1 ; } else { break ; } } for _i in same .. self . namespace . len () { uwrite ! (self . src , "}}\n") ; } self . namespace . truncate (same) ; for i in target . iter () . skip (same) { uwrite ! (self . src , "namespace {} {{\n" , i) ; self . namespace . push (i . clone ()) ; } } fn qualify (& mut self , target : & [String]) { let mut same = 0 ; for (a , b) in self . namespace . iter () . zip (target . iter ()) { if a == b { same += 1 ; } else { break ; } } if same == 0 && ! target . is_empty () { if self . namespace . contains (target . first () . unwrap ()) { self . src . push_str ("::") ; } } if same == target . len () && self . namespace . len () != target . len () && same > 0 { uwrite ! (self . src , "{}::" , target [same - 1]) ; } else { for i in target . iter () . skip (same) { uwrite ! (self . src , "{i}::") ; } } } }
};
}
