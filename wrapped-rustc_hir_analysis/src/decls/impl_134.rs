macro_rules! deps {
    () => {
        IsProbablyCyclical!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for IsProbablyCyclical < 'tcx > { type Result = ControlFlow < () , () > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> ControlFlow < () , () > { let def_id = match ty . kind () { ty :: Adt (adt_def , _) => Some (adt_def . did ()) , ty :: Alias (ty :: Free , alias_ty) => Some (alias_ty . def_id) , _ => None , } ; if let Some (def_id) = def_id { if def_id == self . item_def_id { return ControlFlow :: Break (()) ; } if self . seen . insert (def_id) { self . visit_def (def_id) ? ; } } ty . super_visit_with (self) } }
    };
}

impl_134!();