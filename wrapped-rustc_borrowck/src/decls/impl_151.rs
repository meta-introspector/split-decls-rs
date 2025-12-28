macro_rules! deps {
    () => {
        Locations!();
        FindOpaqueRegion!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for FindOpaqueRegion < '_ , 'tcx > { type Result = ControlFlow < (DefId , usize , Location) , () > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { if let ty :: Alias (ty :: Opaque , opaque) = * ty . kind () && let hir :: OpaqueTyOrigin :: FnReturn { parent , in_trait_or_impl : None } = self . tcx . opaque_ty_origin (opaque . def_id) { let variances = self . tcx . variances_of (opaque . def_id) ; for (idx , (arg , variance)) in std :: iter :: zip (opaque . args , variances) . enumerate () { if * variance == ty :: Bivariant { continue ; } let Some (opaque_region) = arg . as_region () else { continue ; } ; if opaque_region . is_bound () { continue ; } let opaque_region_vid = self . regioncx . to_region_vid (opaque_region) ; if let Some (path) = self . regioncx . constraint_path_between_regions (self . borrow_region , opaque_region_vid) { for constraint in path { if let ConstraintCategory :: CallArgument (Some (call_ty)) = constraint . category && let ty :: FnDef (call_def_id , _) = * call_ty . kind () && call_def_id == parent && let Locations :: Single (location) = constraint . locations { return ControlFlow :: Break ((opaque . def_id , idx , location)) ; } } } } } ty . super_visit_with (self) } }
    };
}

impl_151!()