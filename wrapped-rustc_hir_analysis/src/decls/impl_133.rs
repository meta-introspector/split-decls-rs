macro_rules! deps {
    () => {
        IsProbablyCyclical!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < 'tcx > IsProbablyCyclical < 'tcx > { fn visit_def (& mut self , def_id : DefId) -> ControlFlow < () , () > { match self . tcx . def_kind (def_id) { DefKind :: Struct | DefKind :: Enum | DefKind :: Union => { self . tcx . adt_def (def_id) . all_fields () . try_for_each (| field | { self . tcx . type_of (field . did) . instantiate_identity () . visit_with (self) }) } DefKind :: TyAlias if self . tcx . type_alias_is_lazy (def_id) => { self . tcx . type_of (def_id) . instantiate_identity () . visit_with (self) } _ => ControlFlow :: Continue (()) , } } }
    };
}

impl_133!()