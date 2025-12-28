macro_rules! CoverageInfoBuilderMethods {
    () => {
        pub trait CoverageInfoBuilderMethods < 'tcx > { # [doc = " Handle the MIR coverage info in a backend-specific way."] # [doc = ""] # [doc = " This can potentially be a no-op in backends that don't support"] # [doc = " coverage instrumentation."] fn add_coverage (& mut self , instance : Instance < 'tcx > , kind : & CoverageKind) ; }
    };
}

CoverageInfoBuilderMethods!()