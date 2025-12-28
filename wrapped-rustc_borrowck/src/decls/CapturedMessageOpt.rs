macro_rules! CapturedMessageOpt {
    () => {
        # [doc = " Helper struct for `explain_captures`."] struct CapturedMessageOpt { is_partial_move : bool , is_loop_message : bool , is_move_msg : bool , is_loop_move : bool , has_suggest_reborrow : bool , maybe_reinitialized_locations_is_empty : bool , }
    };
}

CapturedMessageOpt!();