// Generated macro for ArrayIterator (struct)
macro_rules! Depcrate_interpret_projectionArrayIterator {
() => {
// Module: crate::interpret::projection
// Provides: {"ArrayIterator"}
// Dependencies: {}
# [doc = " A type representing iteration over the elements of an array."] pub struct ArrayIterator < 'a , 'tcx , Prov : Provenance , P : Projectable < 'tcx , Prov > > { base : & 'a P , range : Range < u64 > , stride : Size , field_layout : TyAndLayout < 'tcx > , _phantom : PhantomData < Prov > , }
};
}
