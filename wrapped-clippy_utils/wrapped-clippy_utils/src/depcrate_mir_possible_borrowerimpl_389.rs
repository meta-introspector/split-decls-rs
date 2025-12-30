// Generated macro for impl_389 (impl)
macro_rules! Depcrate_mir_possible_borrowerimpl_389 {
() => {
// Module: crate::mir::possible_borrower
// Provides: {"impl_389"}
// Dependencies: {}
impl < 'a , 'b , 'tcx > PossibleBorrowerVisitor < 'a , 'b , 'tcx > { fn new (cx : & 'a LateContext < 'tcx > , body : & 'b mir :: Body < 'tcx > , possible_origin : FxHashMap < mir :: Local , DenseBitSet < mir :: Local > > ,) -> Self { Self { possible_borrower : TransitiveRelation :: default () , body , cx , possible_origin , } } fn into_map (self , cx : & 'a LateContext < 'tcx > , maybe_live : ResultsCursor < 'b , 'tcx , MaybeStorageLive < 'tcx > > ,) -> PossibleBorrowerMap < 'b , 'tcx > { let mut map = FxHashMap :: default () ; for row in (1 .. self . body . local_decls . len ()) . map (mir :: Local :: from_usize) { if is_copy (cx , self . body . local_decls [row] . ty) { continue ; } let mut borrowers = self . possible_borrower . reachable_from (row , self . body . local_decls . len ()) ; borrowers . remove (mir :: Local :: from_usize (0)) ; if ! borrowers . is_empty () { map . insert (row , borrowers) ; } } let bs = DenseBitSet :: new_empty (self . body . local_decls . len ()) ; PossibleBorrowerMap { map , maybe_live , bitset : (bs . clone () , bs) , } } }
};
}
