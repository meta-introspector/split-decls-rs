macro_rules! deps {
    () => {
        Overlap!();
        BorrowData!();
        AccessDepth!();
        BorrowSet!();
        PlaceConflictBias!();
    };
}

macro_rules! each_borrow_involving_path {
    () => {
        deps!();
        # [doc = " Encapsulates the idea of iterating over every borrow that involves a particular path"] pub (super) fn each_borrow_involving_path < 'tcx , F , I , S > (s : & mut S , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , access_place : (AccessDepth , Place < 'tcx >) , borrow_set : & BorrowSet < 'tcx > , is_candidate : I , mut op : F ,) where F : FnMut (& mut S , BorrowIndex , & BorrowData < 'tcx >) -> ControlFlow < () > , I : Fn (BorrowIndex) -> bool , { let (access , place) = access_place ; let Some (borrows_for_place_base) = borrow_set . local_map . get (& place . local) else { return } ; for & i in borrows_for_place_base { if ! is_candidate (i) { continue ; } let borrowed = & borrow_set [i] ; if places_conflict :: borrow_conflicts_with_place (tcx , body , borrowed . borrowed_place , borrowed . kind , place . as_ref () , access , places_conflict :: PlaceConflictBias :: Overlap ,) { debug ! ("each_borrow_involving_path: {:?} @ {:?} vs. {:?}/{:?}" , i , borrowed , place , access) ; let ctrl = op (s , i , borrowed) ; if matches ! (ctrl , ControlFlow :: Break (_)) { return ; } } } }
    };
}

each_borrow_involving_path!()