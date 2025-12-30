// Generated macro for ImplicitHasherConstructorVisitor (struct)
macro_rules! Depcrate_implicit_hasherImplicitHasherConstructorVisitor {
() => {
// Module: crate::implicit_hasher
// Provides: {"ImplicitHasherConstructorVisitor"}
// Dependencies: {}
# [doc = " Looks for default-hasher-dependent constructors like `HashMap::new`."] struct ImplicitHasherConstructorVisitor < 'a , 'b , 'tcx > { cx : & 'a LateContext < 'tcx > , maybe_typeck_results : Option < & 'tcx TypeckResults < 'tcx > > , target : & 'b ImplicitHasherType < 'tcx > , suggestions : BTreeMap < Span , String > , }
};
}
