// Generated macro for impl_713 (impl)
macro_rules! Depcrate_view_memory_layoutimpl_713 {
() => {
// Module: crate::view_memory_layout
// Provides: {"impl_713"}
// Dependencies: {}
impl fmt :: Display for RecursiveMemoryLayout { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fn process (fmt : & mut fmt :: Formatter < '_ > , nodes : & Vec < MemoryLayoutNode > , idx : usize , depth : usize ,) -> fmt :: Result { let mut out = "\t" . repeat (depth) ; let node = & nodes [idx] ; out += & format ! ("{}: {} (size: {}, align: {}, field offset: {})\n" , node . item_name , node . typename , node . size , node . alignment , node . offset) ; write ! (fmt , "{out}") ? ; if node . children_start != - 1 { for j in nodes [idx] . children_start .. (nodes [idx] . children_start + nodes [idx] . children_len as i64) { process (fmt , nodes , j as usize , depth + 1) ? ; } } Ok (()) } process (fmt , & self . nodes , 0 , 0) } }
};
}
