macro_rules! deps {
    () => {
        HasErrorDeep!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for HasErrorDeep < 'tcx > { type Result = ControlFlow < ErrorGuaranteed > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { match * ty . kind () { ty :: Adt (def , _) => { if self . seen . insert (def . did ()) { for field in def . all_fields () { self . tcx . type_of (field . did) . instantiate_identity () . visit_with (self) ? ; } } } ty :: Error (guar) => return ControlFlow :: Break (guar) , _ => { } } ty . super_visit_with (self) } fn visit_region (& mut self , r : ty :: Region < 'tcx >) -> Self :: Result { if let Err (guar) = r . error_reported () { ControlFlow :: Break (guar) } else { ControlFlow :: Continue (()) } } fn visit_const (& mut self , c : ty :: Const < 'tcx >) -> Self :: Result { if let Err (guar) = c . error_reported () { ControlFlow :: Break (guar) } else { ControlFlow :: Continue (()) } } }
    };
}

impl_130!()