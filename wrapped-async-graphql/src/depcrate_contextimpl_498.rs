// Generated macro for impl_498 (impl)
macro_rules! Depcrate_contextimpl_498 {
() => {
// Module: crate::context
// Provides: {"impl_498"}
// Dependencies: {}
impl Debug for SelectionField < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { struct DebugSelectionSet < 'a > (Vec < SelectionField < 'a > >) ; impl Debug for DebugSelectionSet < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_list () . entries (& self . 0) . finish () } } f . debug_struct (self . name ()) . field ("name" , & self . name ()) . field ("selection_set" , & DebugSelectionSet (self . selection_set () . collect ()) ,) . finish () } }
};
}
