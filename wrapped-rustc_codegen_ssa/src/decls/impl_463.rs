macro_rules! deps {
    () => {
        BuilderMethods!();
        FunctionCx!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { pub (crate) fn codegen_coverage (& self , bx : & mut Bx , kind : & CoverageKind , scope : SourceScope) { let instance = if let Some (inlined) = scope . inlined_instance (& self . mir . source_scopes) { self . monomorphize (inlined) } else { self . instance } ; bx . add_coverage (instance , kind) ; } }
    };
}

impl_463!()