macro_rules! deps {
    () => {
        BuilderMethods!();
    };
}

macro_rules! codegen_instance {
    () => {
        deps!();
        pub (crate) fn codegen_instance < 'a , 'tcx : 'a , Bx : BuilderMethods < 'a , 'tcx > > (cx : & 'a Bx :: CodegenCx , instance : Instance < 'tcx > ,) { info ! ("codegen_instance({})" , instance) ; mir :: codegen_mir :: < Bx > (cx , instance) ; }
    };
}

codegen_instance!()