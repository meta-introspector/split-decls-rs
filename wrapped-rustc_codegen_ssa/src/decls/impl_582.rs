macro_rules! deps {
    () => {
        MiscCodegenMethods!();
        BaseTypeCodegenMethods!();
        DerivedTypeCodegenMethods!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl < 'tcx , T > DerivedTypeCodegenMethods < 'tcx > for T where Self : BaseTypeCodegenMethods + MiscCodegenMethods < 'tcx > + HasTyCtxt < 'tcx > + HasTypingEnv < 'tcx > { }
    };
}

impl_582!()