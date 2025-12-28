macro_rules! deps {
    () => {
        Utf8Error!();
    };
}

macro_rules! validate {
    () => {
        deps!();
        # [doc = " Returns OK if and only if the given slice is completely valid UTF-8."] # [doc = ""] # [doc = " If the slice isn't valid UTF-8, then an error is returned that explains"] # [doc = " the first location at which invalid UTF-8 was detected."] pub fn validate (slice : & [u8]) -> Result < () , Utf8Error > { fn fast (slice : & [u8]) -> Result < () , Utf8Error > { let mut state = ACCEPT ; let mut i = 0 ; while i < slice . len () { let b = slice [i] ; if state == ACCEPT && b <= 0x7F && slice . get (i + 1) . map_or (false , | & b | b <= 0x7F) { i += ascii :: first_non_ascii_byte (& slice [i ..]) ; continue ; } state = step (state , b) ; if state == REJECT { return Err (find_valid_up_to (slice , i)) ; } i += 1 ; } if state != ACCEPT { Err (find_valid_up_to (slice , slice . len ())) } else { Ok (()) } } # [inline (never)] fn find_valid_up_to (slice : & [u8] , rejected_at : usize) -> Utf8Error { let mut backup = rejected_at . saturating_sub (1) ; while backup > 0 && ! is_leading_or_invalid_utf8_byte (slice [backup]) { backup -= 1 ; } let upto = cmp :: min (slice . len () , rejected_at . saturating_add (1)) ; let mut err = slow (& slice [backup .. upto]) . unwrap_err () ; err . valid_up_to += backup ; err } fn slow (slice : & [u8]) -> Result < () , Utf8Error > { let mut state = ACCEPT ; let mut valid_up_to = 0 ; for (i , & b) in slice . iter () . enumerate () { state = step (state , b) ; if state == ACCEPT { valid_up_to = i + 1 ; } else if state == REJECT { let error_len = Some (cmp :: max (1 , i - valid_up_to)) ; return Err (Utf8Error { valid_up_to , error_len }) ; } } if state != ACCEPT { Err (Utf8Error { valid_up_to , error_len : None }) } else { Ok (()) } } fn step (state : usize , b : u8) -> usize { let class = CLASSES [b as usize] ; unsafe { * STATES_FORWARD . get_unchecked (state + class as usize) as usize } } fast (slice) }
    };
}

validate!();