macro_rules! deps {
    () => {
        StaticCodegenMethods!();
        PreDefineCodegenMethods!();
        ConstCodegenMethods!();
        AsmCodegenMethods!();
        DebugInfoCodegenMethods!();
    };
}

macro_rules! other_593 {
    () => {
        deps!();
        pub trait CodegenMethods < 'tcx > = LayoutOf < 'tcx , LayoutOfResult = TyAndLayout < 'tcx > > + FnAbiOf < 'tcx , FnAbiOfResult = & 'tcx FnAbi < 'tcx , Ty < 'tcx > > > + TypeCodegenMethods < 'tcx > + ConstCodegenMethods + StaticCodegenMethods + DebugInfoCodegenMethods < 'tcx > + AsmCodegenMethods < 'tcx > + PreDefineCodegenMethods < 'tcx > ;
    };
}

other_593!();