macro_rules! restrict_repr_packed_field_ref_capture {
    () => {
        # [doc = " Truncate the capture so that the place being borrowed is in accordance with RFC 1240,"] # [doc = " which states that it's unsafe to take a reference into a struct marked `repr(packed)`."] fn restrict_repr_packed_field_ref_capture < 'tcx > (mut place : Place < 'tcx > , mut curr_borrow_kind : ty :: UpvarCapture ,) -> (Place < 'tcx > , ty :: UpvarCapture) { let pos = place . projections . iter () . enumerate () . position (| (i , p) | { let ty = place . ty_before_projection (i) ; match p . kind { ProjectionKind :: Field (..) => match ty . kind () { ty :: Adt (def , _) if def . repr () . packed () => { true } _ => false , } , _ => false , } }) ; if let Some (pos) = pos { truncate_place_to_len_and_update_capture_kind (& mut place , & mut curr_borrow_kind , pos) ; } (place , curr_borrow_kind) }
    };
}

restrict_repr_packed_field_ref_capture!();