macro_rules! deps {
    () => {
        GatherBorrows!();
        TwoPhaseActivation!();
        BorrowData!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'a , 'tcx > Visitor < 'tcx > for GatherBorrows < 'a , 'tcx > { fn visit_assign (& mut self , assigned_place : & mir :: Place < 'tcx > , rvalue : & mir :: Rvalue < 'tcx > , location : mir :: Location ,) { if let & mir :: Rvalue :: Ref (region , kind , borrowed_place) = rvalue { if borrowed_place . ignore_borrow (self . tcx , self . body , & self . locals_state_at_exit) { debug ! ("ignoring_borrow of {:?}" , borrowed_place) ; return ; } let region = region . as_var () ; let borrow = BorrowData { kind , region , reserve_location : location , activation_location : TwoPhaseActivation :: NotTwoPhase , borrowed_place , assigned_place : * assigned_place , } ; let (idx , _) = self . location_map . insert_full (location , borrow) ; let idx = BorrowIndex :: from (idx) ; self . insert_as_pending_if_two_phase (location , assigned_place , kind , idx) ; self . local_map . entry (borrowed_place . local) . or_default () . insert (idx) ; } self . super_assign (assigned_place , rvalue , location) } fn visit_local (& mut self , temp : Local , context : PlaceContext , location : Location) { if ! context . is_use () { return ; } if let Some (& borrow_index) = self . pending_activations . get (& temp) { let borrow_data = & mut self . location_map [borrow_index . as_usize ()] ; if borrow_data . reserve_location == location && context == PlaceContext :: MutatingUse (MutatingUseContext :: Store) { return ; } if let TwoPhaseActivation :: ActivatedAt (other_location) = borrow_data . activation_location { span_bug ! (self . body . source_info (location) . span , "found two uses for 2-phase borrow temporary {:?}: \
                     {:?} and {:?}" , temp , location , other_location ,) ; } assert_eq ! (borrow_data . activation_location , TwoPhaseActivation :: NotActivated , "never found an activation for this borrow!" ,) ; self . activation_map . entry (location) . or_default () . push (borrow_index) ; borrow_data . activation_location = TwoPhaseActivation :: ActivatedAt (location) ; } } fn visit_rvalue (& mut self , rvalue : & mir :: Rvalue < 'tcx > , location : mir :: Location) { if let & mir :: Rvalue :: Ref (region , kind , place) = rvalue { let borrow_data = & self . location_map [& location] ; assert_eq ! (borrow_data . reserve_location , location) ; assert_eq ! (borrow_data . kind , kind) ; assert_eq ! (borrow_data . region , region . as_var ()) ; assert_eq ! (borrow_data . borrowed_place , place) ; } self . super_rvalue (rvalue , location) } }
    };
}

impl_11!();