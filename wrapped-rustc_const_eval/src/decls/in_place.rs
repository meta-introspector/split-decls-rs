macro_rules! deps {
    () => {
        Place!();
        ConstCx!();
        Qualif!();
    };
}

macro_rules! in_place {
    () => {
        deps!();
        # [doc = " Returns `true` if this `Place` contains qualif `Q`."] pub fn in_place < 'tcx , Q , F > (cx : & ConstCx < '_ , 'tcx > , in_local : & mut F , place : PlaceRef < 'tcx >) -> bool where Q : Qualif , F : FnMut (Local) -> bool , { let mut place = place ; while let Some ((place_base , elem)) = place . last_projection () { match elem { ProjectionElem :: Index (index) if in_local (index) => return true , ProjectionElem :: Deref | ProjectionElem :: Subtype (_) | ProjectionElem :: Field (_ , _) | ProjectionElem :: OpaqueCast (_) | ProjectionElem :: ConstantIndex { .. } | ProjectionElem :: Subslice { .. } | ProjectionElem :: Downcast (_ , _) | ProjectionElem :: Index (_) | ProjectionElem :: UnwrapUnsafeBinder (_) => { } } let base_ty = place_base . ty (cx . body , cx . tcx) ; let proj_ty = base_ty . projection_ty (cx . tcx , elem) . ty ; if ! Q :: in_any_value_of_ty (cx , proj_ty) { return false ; } if matches ! (elem , ProjectionElem :: Deref) { return true ; } place = place_base ; } assert ! (place . projection . is_empty ()) ; in_local (place . local) }
    };
}

in_place!();