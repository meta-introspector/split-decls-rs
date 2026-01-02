mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_abi :: FieldIdx ;}
mkuse!{use rustc_data_structures :: graph :: dominators :: Dominators ;}
mkuse!{use rustc_middle :: mir :: { BasicBlock , Body , Location , Place , PlaceRef , ProjectionElem } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: borrow_set :: { BorrowData , BorrowSet , TwoPhaseActivation } ;}
mkuse!{use crate :: { AccessDepth , BorrowIndex , places_conflict } ;}

macro_rules! each_borrow_involving_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function each_borrow_involving_path in module {}", module_path!());
    };
}

mkfn!{
    each_borrow_involving_path_introspect!();
    # [doc = " Encapsulates the idea of iterating over every borrow that involves a particular path"] pub (super) fn each_borrow_involving_path < 'tcx , F , I , S > (s : & mut S , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , access_place : (AccessDepth , Place < 'tcx >) , borrow_set : & BorrowSet < 'tcx > , is_candidate : I , mut op : F ,) where F : FnMut (& mut S , BorrowIndex , & BorrowData < 'tcx >) -> ControlFlow < () > , I : Fn (BorrowIndex) -> bool , { let (access , place) = access_place ; let Some (borrows_for_place_base) = borrow_set . local_map . get (& place . local) else { return } ; for & i in borrows_for_place_base { if ! is_candidate (i) { continue ; } let borrowed = & borrow_set [i] ; if places_conflict :: borrow_conflicts_with_place (tcx , body , borrowed . borrowed_place , borrowed . kind , place . as_ref () , access , places_conflict :: PlaceConflictBias :: Overlap ,) { debug ! ("each_borrow_involving_path: {:?} @ {:?} vs. {:?}/{:?}" , i , borrowed , place , access) ; let ctrl = op (s , i , borrowed) ; if matches ! (ctrl , ControlFlow :: Break (_)) { return ; } } } }
}

macro_rules! is_active_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_active in module {}", module_path!());
    };
}

mkfn!{
    is_active_introspect!();
    pub (super) fn is_active < 'tcx > (dominators : & Dominators < BasicBlock > , borrow_data : & BorrowData < 'tcx > , location : Location ,) -> bool { debug ! ("is_active(borrow_data={:?}, location={:?})" , borrow_data , location) ; let activation_location = match borrow_data . activation_location { TwoPhaseActivation :: NotTwoPhase => return true , TwoPhaseActivation :: NotActivated => return false , TwoPhaseActivation :: ActivatedAt (loc) => loc , } ; if activation_location . dominates (location , dominators) { return true ; } let reserve_location = borrow_data . reserve_location . successor_within_block () ; if reserve_location . dominates (location , dominators) { false } else { true } }
}

macro_rules! borrow_of_local_data_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function borrow_of_local_data in module {}", module_path!());
    };
}

mkfn!{
    borrow_of_local_data_introspect!();
    # [doc = " Determines if a given borrow is borrowing local data"] # [doc = " This is called for all Yield expressions on movable coroutines"] pub (super) fn borrow_of_local_data (place : Place < '_ >) -> bool { ! place . is_indirect () }
}

macro_rules! is_upvar_field_projection_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_upvar_field_projection in module {}", module_path!());
    };
}

mkfn!{
    is_upvar_field_projection_introspect!();
    # [doc = " If `place` is a field projection, and the field is being projected from a closure type,"] # [doc = " then returns the index of the field being projected. Note that this closure will always"] # [doc = " be `self` in the current MIR, because that is the only time we directly access the fields"] # [doc = " of a closure type."] pub (crate) fn is_upvar_field_projection < 'tcx > (tcx : TyCtxt < 'tcx > , upvars : & [& rustc_middle :: ty :: CapturedPlace < 'tcx >] , place_ref : PlaceRef < 'tcx > , body : & Body < 'tcx > ,) -> Option < FieldIdx > { let mut place_ref = place_ref ; let mut by_ref = false ; if let Some ((place_base , ProjectionElem :: Deref)) = place_ref . last_projection () { place_ref = place_base ; by_ref = true ; } match place_ref . last_projection () { Some ((place_base , ProjectionElem :: Field (field , _ty))) => { let base_ty = place_base . ty (body , tcx) . ty ; if (base_ty . is_closure () || base_ty . is_coroutine () || base_ty . is_coroutine_closure ()) && (! by_ref || upvars [field . index ()] . is_by_ref ()) { Some (field) } else { None } } _ => None , } }
}