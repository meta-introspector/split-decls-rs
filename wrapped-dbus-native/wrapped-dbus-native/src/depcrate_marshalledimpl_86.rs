// Generated macro for impl_86 (impl)
macro_rules! Depcrate_marshalledimpl_86 {
() => {
// Module: crate::marshalled
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a > Iterator for Dict < 'a > { type Item = Result < (Single < 'a > , Single < 'a >) , DemarshalError > ; fn next (& mut self) -> Option < Self :: Item > { if self . data . len () == 0 { return None ; } let mut mi = MultiIter { start_pos : 0 , inner : Multi { sig : SignatureMulti :: new_unchecked (& self . outer_sig [2 .. self . outer_sig . len () - 1]) , data : self . data , is_big_endian : self . is_big_endian , } } ; match (mi . next () , mi . next ()) { (Some (Ok (k)) , Some (Ok (v))) => { let len = self . data . len () - mi . inner . data . len () ; if len < self . data . len () { self . data = & self . data [align_up (len , 8) ..] ; } else { self . data = & [] ; } Some (Ok ((k , v))) } , (Some (Err (k)) , Some (_)) => Some (Err (k)) , (Some (_) , Some (Err (v))) => Some (Err (v)) , _ => { Some (Err (DemarshalError :: NotEnoughData)) } , } } }
};
}
