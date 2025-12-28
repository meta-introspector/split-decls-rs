macro_rules! deps {
    () => {
        AccessDepth!();
        PlaceConflictBias!();
    };
}

macro_rules! borrow_conflicts_with_place {
    () => {
        deps!();
        # [doc = " Checks whether the `borrow_place` conflicts with the `access_place` given a borrow kind and"] # [doc = " access depth. The `bias` parameter is used to determine how the unknowable (comparing runtime"] # [doc = " array indices, for example) should be interpreted - this depends on what the caller wants in"] # [doc = " order to make the conservative choice and preserve soundness."] # [inline] pub (super) fn borrow_conflicts_with_place < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , borrow_place : Place < 'tcx > , borrow_kind : BorrowKind , access_place : PlaceRef < 'tcx > , access : AccessDepth , bias : PlaceConflictBias ,) -> bool { let borrow_local = borrow_place . local ; let access_local = access_place . local ; if borrow_local != access_local { return false ; } if borrow_place . projection . is_empty () && access_place . projection . is_empty () { return true ; } place_components_conflict (tcx , body , borrow_place , borrow_kind , access_place , access , bias) }
    };
}

borrow_conflicts_with_place!()