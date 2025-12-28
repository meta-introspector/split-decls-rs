macro_rules! deps {
    () => {
        HasRecursiveOpaque!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for HasRecursiveOpaque < '_ , 'tcx > { type Result = ControlFlow < () > ; fn visit_ty (& mut self , t : Ty < 'tcx >) -> Self :: Result { if let ty :: Alias (ty :: Opaque , alias_ty) = * t . kind () && let Some (def_id) = alias_ty . def_id . as_local () { if self . def_id == def_id { return ControlFlow :: Break (()) ; } if self . seen . insert (def_id) && let Some (hidden_ty) = self . opaques . get (& def_id) { ty :: EarlyBinder :: bind (hidden_ty . ty) . instantiate (self . tcx , alias_ty . args) . visit_with (self) ? ; } } t . super_visit_with (self) } }
    };
}

impl_391!()