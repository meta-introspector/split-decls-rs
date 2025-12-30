// Generated macro for impl_discrete_trait (macro)
macro_rules! Depcrate_coord_ranged1d_types_numericimpl_discrete_trait {
() => {
// Module: crate::coord::ranged1d::types::numeric
// Provides: {"impl_discrete_trait"}
// Dependencies: {}
macro_rules ! impl_discrete_trait { ($ name : ident) => { impl DiscreteRanged for $ name { fn size (& self) -> usize { if & self . 1 < & self . 0 { return 0 ; } let values = self . 1 - self . 0 ; (values + 1) as usize } fn index_of (& self , value : & Self :: ValueType) -> Option < usize > { if value < & self . 0 { return None ; } let ret = value - self . 0 ; Some (ret as usize) } fn from_index (& self , index : usize) -> Option < Self :: ValueType > { Self :: ValueType :: try_from (index) . ok () . and_then (| index | self . 0 . checked_add (index)) } } } ; }
};
}
