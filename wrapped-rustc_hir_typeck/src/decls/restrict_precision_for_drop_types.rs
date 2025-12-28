macro_rules! deps {
    () => {
        FnCtxt!();
    };
}

macro_rules! restrict_precision_for_drop_types {
    () => {
        deps!();
        # [doc = " Rust doesn't permit moving fields out of a type that implements drop"] fn restrict_precision_for_drop_types < 'a , 'tcx > (fcx : & 'a FnCtxt < 'a , 'tcx > , mut place : Place < 'tcx > , mut curr_mode : ty :: UpvarCapture ,) -> (Place < 'tcx > , ty :: UpvarCapture) { let is_copy_type = fcx . infcx . type_is_copy_modulo_regions (fcx . param_env , place . ty ()) ; if let (false , UpvarCapture :: ByValue) = (is_copy_type , curr_mode) { for i in 0 .. place . projections . len () { match place . ty_before_projection (i) . kind () { ty :: Adt (def , _) if def . destructor (fcx . tcx) . is_some () => { truncate_place_to_len_and_update_capture_kind (& mut place , & mut curr_mode , i) ; break ; } _ => { } } } } (place , curr_mode) }
    };
}

restrict_precision_for_drop_types!()