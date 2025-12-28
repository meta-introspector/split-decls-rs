macro_rules! deps {
    () => {
        ClosureOutlivesSubjectTy!();
    };
}

macro_rules! impl_527 {
    () => {
        deps!();
        impl < 'tcx , I > ! TypeVisitable < I > for ClosureOutlivesSubjectTy < 'tcx > { }
    };
}

impl_527!()