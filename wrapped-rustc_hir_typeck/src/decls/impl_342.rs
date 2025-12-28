macro_rules! deps {
    () => {
        InferBorrowKindVisitor!();
        FnCtxt!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { pub (crate) fn closure_analyze (& self , body : & 'tcx hir :: Body < 'tcx >) { InferBorrowKindVisitor { fcx : self } . visit_body (body) ; assert ! (self . deferred_call_resolutions . borrow () . is_empty ()) ; } }
    };
}

impl_342!();