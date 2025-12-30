// Generated macro for impl_755 (impl)
macro_rules! Depcrate_csrimpl_755 {
() => {
// Module: crate::csr
// Provides: {"impl_755"}
// Dependencies: {}
impl < 'a , E , Ty , Ix > Iterator for EdgeReferences < 'a , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Item = EdgeReference < 'a , E , Ty , Ix > ; fn next (& mut self) -> Option < Self :: Item > { loop { if let Some ((& j , w)) = self . iter . next () { let index = self . index ; self . index += 1 ; return Some (EdgeReference { index , source : self . source_index , target : j , weight : w , ty : PhantomData , }) ; } if let Some ((i , w)) = self . edge_ranges . next () { let a = w [0] ; let b = w [1] ; self . iter = zip (& self . column [a .. b] , & self . edges [a .. b]) ; self . source_index = Ix :: new (i) ; } else { return None ; } } } }
};
}
