// Generated macro for default_optimisation_attrs (function)
macro_rules! Depcrate_attributesdefault_optimisation_attrs {
() => {
// Module: crate::attributes
// Provides: {"default_optimisation_attrs"}
// Dependencies: {}
# [doc = " Get the default optimizations attrs for a function."] # [inline] pub (crate) fn default_optimisation_attrs < 'll > (cx : & CodegenCx < 'll , '_ > ,) -> SmallVec < [& 'll Attribute ; 2] > { let mut attrs = SmallVec :: new () ; match cx . sess () . opts . optimize { OptLevel :: Size => { attrs . push (llvm :: AttributeKind :: OptimizeForSize . create_attr (cx . llcx)) ; } OptLevel :: SizeMin => { attrs . push (llvm :: AttributeKind :: MinSize . create_attr (cx . llcx)) ; attrs . push (llvm :: AttributeKind :: OptimizeForSize . create_attr (cx . llcx)) ; } _ => { } } attrs }
};
}
