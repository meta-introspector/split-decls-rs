macro_rules! adjust_for_use_closure {
    () => {
        # [doc = " Truncate deref of any reference."] fn adjust_for_use_closure (mut place : Place < '_ > , mut kind : ty :: UpvarCapture ,) -> (Place < '_ > , ty :: UpvarCapture) { let first_deref = place . projections . iter () . position (| proj | proj . kind == ProjectionKind :: Deref) ; if let Some (idx) = first_deref { truncate_place_to_len_and_update_capture_kind (& mut place , & mut kind , idx) ; } (place , ty :: UpvarCapture :: ByUse) }
    };
}

adjust_for_use_closure!()