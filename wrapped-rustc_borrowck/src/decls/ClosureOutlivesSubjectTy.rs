macro_rules! deps {
    () => {
        ClosureOutlivesSubject!();
    };
}

macro_rules! ClosureOutlivesSubjectTy {
    () => {
        deps!();
        # [doc = " Represents a `ty::Ty` for use in [`ClosureOutlivesSubject`]."] # [doc = ""] # [doc = " This abstraction is necessary because the type may include `ReVar` regions,"] # [doc = " which is what we use internally within NLL code, and they can't be used in"] # [doc = " a query response."] # [derive (Copy , Clone , Debug)] pub struct ClosureOutlivesSubjectTy < 'tcx > { inner : Ty < 'tcx > , }
    };
}

ClosureOutlivesSubjectTy!();