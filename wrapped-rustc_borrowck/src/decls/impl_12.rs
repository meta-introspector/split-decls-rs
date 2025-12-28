macro_rules! deps {
    () => {
        TwoPhaseActivation!();
        GatherBorrows!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < 'a , 'tcx > GatherBorrows < 'a , 'tcx > { # [doc = " If this is a two-phase borrow, then we will record it"] # [doc = " as \"pending\" until we find the activating use."] fn insert_as_pending_if_two_phase (& mut self , start_location : Location , assigned_place : & mir :: Place < 'tcx > , kind : mir :: BorrowKind , borrow_index : BorrowIndex ,) { debug ! ("Borrows::insert_as_pending_if_two_phase({:?}, {:?}, {:?})" , start_location , assigned_place , borrow_index ,) ; if ! kind . allows_two_phase_borrow () { debug ! ("  -> {:?}" , start_location) ; return ; } let Some (temp) = assigned_place . as_local () else { span_bug ! (self . body . source_info (start_location) . span , "expected 2-phase borrow to assign to a local, not `{:?}`" , assigned_place ,) ; } ; { let borrow_data = & mut self . location_map [borrow_index . as_usize ()] ; borrow_data . activation_location = TwoPhaseActivation :: NotActivated ; } let old_value = self . pending_activations . insert (temp , borrow_index) ; if let Some (old_index) = old_value { span_bug ! (self . body . source_info (start_location) . span , "found already pending activation for temp: {:?} \
                       at borrow_index: {:?} with associated data {:?}" , temp , old_index , self . location_map [old_index . as_usize ()]) ; } } }
    };
}

impl_12!();