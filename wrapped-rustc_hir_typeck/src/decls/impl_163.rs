macro_rules! deps {
    () => {
        ExprUseVisitor!();
        Delegate!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < 'a , 'tcx , D : Delegate < 'tcx > > ExprUseVisitor < 'tcx , (& 'a LateContext < 'tcx > , LocalDefId) , D > { pub fn for_clippy (cx : & 'a LateContext < 'tcx > , body_def_id : LocalDefId , delegate : D) -> Self { Self :: new ((cx , body_def_id) , delegate) } }
    };
}

impl_163!()