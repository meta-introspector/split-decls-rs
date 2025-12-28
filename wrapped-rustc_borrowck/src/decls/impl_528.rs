macro_rules! deps {
    () => {
        ClosureOutlivesSubjectTy!();
    };
}

macro_rules! impl_528 {
    () => {
        deps!();
        impl < 'tcx , I > ! TypeFoldable < I > for ClosureOutlivesSubjectTy < 'tcx > { }
    };
}

impl_528!()