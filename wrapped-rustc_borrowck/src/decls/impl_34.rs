macro_rules! deps {
    () => {
        ClosureOutlivesSubjectTy!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'tcx , I > ! TypeFoldable < I > for ClosureOutlivesSubjectTy < 'tcx > { }
    };
}

impl_34!()