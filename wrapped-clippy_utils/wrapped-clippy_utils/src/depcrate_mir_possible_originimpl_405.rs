// Generated macro for impl_405 (impl)
macro_rules! Depcrate_mir_possible_originimpl_405 {
() => {
// Module: crate::mir::possible_origin
// Provides: {"impl_405"}
// Dependencies: {}
impl < 'a , 'tcx > PossibleOriginVisitor < 'a , 'tcx > { pub fn new (body : & 'a mir :: Body < 'tcx >) -> Self { Self { possible_origin : TransitiveRelation :: default () , body , } } pub fn into_map (self , cx : & LateContext < 'tcx >) -> FxHashMap < mir :: Local , DenseBitSet < mir :: Local > > { let mut map = FxHashMap :: default () ; for row in (1 .. self . body . local_decls . len ()) . map (mir :: Local :: from_usize) { if is_copy (cx , self . body . local_decls [row] . ty) { continue ; } let mut borrowers = self . possible_origin . reachable_from (row , self . body . local_decls . len ()) ; borrowers . remove (mir :: Local :: from_usize (0)) ; if ! borrowers . is_empty () { map . insert (row , borrowers) ; } } map } }
};
}
