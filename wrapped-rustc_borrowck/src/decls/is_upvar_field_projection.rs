macro_rules! is_upvar_field_projection {
    () => {
        # [doc = " If `place` is a field projection, and the field is being projected from a closure type,"] # [doc = " then returns the index of the field being projected. Note that this closure will always"] # [doc = " be `self` in the current MIR, because that is the only time we directly access the fields"] # [doc = " of a closure type."] pub (crate) fn is_upvar_field_projection < 'tcx > (tcx : TyCtxt < 'tcx > , upvars : & [& rustc_middle :: ty :: CapturedPlace < 'tcx >] , place_ref : PlaceRef < 'tcx > , body : & Body < 'tcx > ,) -> Option < FieldIdx > { let mut place_ref = place_ref ; let mut by_ref = false ; if let Some ((place_base , ProjectionElem :: Deref)) = place_ref . last_projection () { place_ref = place_base ; by_ref = true ; } match place_ref . last_projection () { Some ((place_base , ProjectionElem :: Field (field , _ty))) => { let base_ty = place_base . ty (body , tcx) . ty ; if (base_ty . is_closure () || base_ty . is_coroutine () || base_ty . is_coroutine_closure ()) && (! by_ref || upvars [field . index ()] . is_by_ref ()) { Some (field) } else { None } } _ => None , } }
    };
}

is_upvar_field_projection!()