macro_rules! deps {
    () => {
        CodegenCx!();
        SmallVec!();
        AttributeKind!();
    };
}

macro_rules! default_optimisation_attrs {
    () => {
        deps!();
        # [doc = " Get the default optimizations attrs for a function."] # [inline] pub (crate) fn default_optimisation_attrs < 'll > (cx : & CodegenCx < 'll , '_ > ,) -> SmallVec < [& 'll Attribute ; 2] > { let mut attrs = SmallVec :: new () ; match cx . sess () . opts . optimize { OptLevel :: Size => { attrs . push (llvm :: AttributeKind :: OptimizeForSize . create_attr (cx . llcx)) ; } OptLevel :: SizeMin => { attrs . push (llvm :: AttributeKind :: MinSize . create_attr (cx . llcx)) ; attrs . push (llvm :: AttributeKind :: OptimizeForSize . create_attr (cx . llcx)) ; } _ => { } } attrs }
    };
}

default_optimisation_attrs!()