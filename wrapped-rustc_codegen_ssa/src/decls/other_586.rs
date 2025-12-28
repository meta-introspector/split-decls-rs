macro_rules! deps {
    () => {
        TypeMembershipCodegenMethods!();
        LayoutTypeCodegenMethods!();
        DerivedTypeCodegenMethods!();
    };
}

macro_rules! other_586 {
    () => {
        deps!();
        pub trait TypeCodegenMethods < 'tcx > = DerivedTypeCodegenMethods < 'tcx > + LayoutTypeCodegenMethods < 'tcx > + TypeMembershipCodegenMethods < 'tcx > ;
    };
}

other_586!();