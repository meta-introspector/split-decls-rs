macro_rules! deps {
    () => {
        DebugInfoCodegenMethods!();
        AsmCodegenMethods!();
        PreDefineCodegenMethods!();
        ConstCodegenMethods!();
        StaticCodegenMethods!();
    };
}

macro_rules! other_593 {
    () => {
        deps!();
        pub trait CodegenMethods < 'tcx > = LayoutOf < 'tcx , LayoutOfResult = TyAndLayout < 'tcx > > + FnAbiOf < 'tcx , FnAbiOfResult = & 'tcx FnAbi < 'tcx , Ty < 'tcx > > > + TypeCodegenMethods < 'tcx > + ConstCodegenMethods + StaticCodegenMethods + DebugInfoCodegenMethods < 'tcx > + AsmCodegenMethods < 'tcx > + PreDefineCodegenMethods < 'tcx > ;
    };
}

other_593!()