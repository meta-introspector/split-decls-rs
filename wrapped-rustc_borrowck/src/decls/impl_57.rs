macro_rules! deps {
    () => {
        PoloniusOutOfScopePrecomputer!();
        PlaceConflictBias!();
        RegionInferenceContext!();
        BorrowSet!();
        Borrows!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'a , 'tcx > Borrows < 'a , 'tcx > { pub fn new (tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , borrow_set : & 'a BorrowSet < 'tcx > ,) -> Self { let borrows_out_of_scope_at_location = if ! tcx . sess . opts . unstable_opts . polonius . is_next_enabled () { calculate_borrows_out_of_scope_at_location (body , regioncx , borrow_set) } else { PoloniusOutOfScopePrecomputer :: compute (body , regioncx , borrow_set) } ; Borrows { tcx , body , borrow_set , borrows_out_of_scope_at_location } } # [doc = " Add all borrows to the kill set, if those borrows are out of scope at `location`."] # [doc = " That means they went out of a nonlexical scope"] fn kill_loans_out_of_scope_at_location (& self , state : & mut < Self as Analysis < 'tcx > > :: Domain , location : Location ,) { if let Some (indices) = self . borrows_out_of_scope_at_location . get (& location) { state . kill_all (indices . iter () . copied ()) ; } } # [doc = " Kill any borrows that conflict with `place`."] fn kill_borrows_on_place (& self , state : & mut < Self as Analysis < 'tcx > > :: Domain , place : Place < 'tcx > ,) { debug ! ("kill_borrows_on_place: place={:?}" , place) ; let other_borrows_of_local = self . borrow_set . local_map . get (& place . local) . into_iter () . flat_map (| bs | bs . iter ()) . copied () ; if place . projection . is_empty () { if ! self . body . local_decls [place . local] . is_ref_to_static () { state . kill_all (other_borrows_of_local) ; } return ; } let definitely_conflicting_borrows = other_borrows_of_local . filter (| & i | { places_conflict (self . tcx , self . body , self . borrow_set [i] . borrowed_place , place , PlaceConflictBias :: NoOverlap ,) }) ; state . kill_all (definitely_conflicting_borrows) ; } }
    };
}

impl_57!()