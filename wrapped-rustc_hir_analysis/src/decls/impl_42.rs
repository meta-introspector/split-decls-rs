macro_rules! deps {
    () => {
        RemapLateParam!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for RemapLateParam < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_region (& mut self , r : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { if let ty :: ReLateParam (fr) = r . kind () { ty :: Region :: new_late_param (self . tcx , fr . scope , self . mapping . get (& fr . kind) . copied () . unwrap_or (fr . kind) ,) } else { r } } }
    };
}

impl_42!();