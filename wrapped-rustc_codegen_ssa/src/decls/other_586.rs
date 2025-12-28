macro_rules! deps {
    () => {
        LayoutTypeCodegenMethods!();
        DerivedTypeCodegenMethods!();
        TypeMembershipCodegenMethods!();
    };
}

macro_rules! other_586 {
    () => {
        deps!();
        pub trait TypeCodegenMethods < 'tcx > = DerivedTypeCodegenMethods < 'tcx > + LayoutTypeCodegenMethods < 'tcx > + TypeMembershipCodegenMethods < 'tcx > ;
    };
}

other_586!()