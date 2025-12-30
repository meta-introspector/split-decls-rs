// Generated macro for impl_523 (impl)
macro_rules! Depcrate_vectorimpl_523 {
() => {
// Module: crate::vector
// Provides: {"impl_523"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < A : Clone + Eq > PartialEq for Vector < A > { fn eq (& self , other : & Self) -> bool { fn cmp_chunk < A > (left : & PoolRef < Chunk < A > > , right : & PoolRef < Chunk < A > >) -> bool { (left . is_empty () && right . is_empty ()) || PoolRef :: ptr_eq (left , right) } if std :: ptr :: eq (self , other) { return true ; } match (& self . vector , & other . vector) { (Single (_ , left) , Single (_ , right)) => { if cmp_chunk (left , right) { return true ; } self . iter () . eq (other . iter ()) } (Full (_ , left) , Full (_ , right)) => { if left . length != right . length { return false ; } if cmp_chunk (& left . outer_f , & right . outer_f) && cmp_chunk (& left . inner_f , & right . inner_f) && cmp_chunk (& left . inner_b , & right . inner_b) && cmp_chunk (& left . outer_b , & right . outer_b) && ((left . middle . is_empty () && right . middle . is_empty ()) || Ref :: ptr_eq (& left . middle , & right . middle)) { return true ; } self . iter () . eq (other . iter ()) } _ => self . len () == other . len () && self . iter () . eq (other . iter ()) , } } }
};
}
