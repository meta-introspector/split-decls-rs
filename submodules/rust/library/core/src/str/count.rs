mkuse!{use core :: intrinsics :: unlikely ;}
mkitem!{const USIZE_SIZE : usize = size_of :: < usize > () ;}
mkitem!{const UNROLL_INNER : usize = 4 ;}

macro_rules! count_chars_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function count_chars in module {}", module_path!());
    };
}

mkfn!{
    count_chars_introspect!();
    # [inline] pub (super) fn count_chars (s : & str) -> usize { if cfg ! (feature = "optimize_for_size") || s . len () < USIZE_SIZE * UNROLL_INNER { char_count_general_case (s . as_bytes ()) } else { do_count_chars (s) } }
}

macro_rules! do_count_chars_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function do_count_chars in module {}", module_path!());
    };
}

mkfn!{
    do_count_chars_introspect!();
    fn do_count_chars (s : & str) -> usize { const CHUNK_SIZE : usize = 192 ; const _ : () = assert ! (CHUNK_SIZE < 256) ; const _ : () = assert ! (CHUNK_SIZE . is_multiple_of (UNROLL_INNER)) ; let (head , body , tail) = unsafe { s . as_bytes () . align_to :: < usize > () } ; if unlikely (body . is_empty () || head . len () > USIZE_SIZE || tail . len () > USIZE_SIZE) { return char_count_general_case (s . as_bytes ()) ; } let mut total = char_count_general_case (head) + char_count_general_case (tail) ; for chunk in body . chunks (CHUNK_SIZE) { let mut counts = 0 ; let (unrolled_chunks , remainder) = chunk . as_chunks :: < UNROLL_INNER > () ; for unrolled in unrolled_chunks { for & word in unrolled { counts += contains_non_continuation_byte (word) ; } } total += sum_bytes_in_usize (counts) ; if ! remainder . is_empty () { let mut counts = 0 ; for & word in remainder { counts += contains_non_continuation_byte (word) ; } total += sum_bytes_in_usize (counts) ; break ; } } total }
}

macro_rules! contains_non_continuation_byte_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function contains_non_continuation_byte in module {}", module_path!());
    };
}

mkfn!{
    contains_non_continuation_byte_introspect!();
    # [inline] fn contains_non_continuation_byte (w : usize) -> usize { const LSB : usize = usize :: repeat_u8 (0x01) ; ((! w >> 7) | (w >> 6)) & LSB }
}

macro_rules! sum_bytes_in_usize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sum_bytes_in_usize in module {}", module_path!());
    };
}

mkfn!{
    sum_bytes_in_usize_introspect!();
    # [inline] fn sum_bytes_in_usize (values : usize) -> usize { const LSB_SHORTS : usize = usize :: repeat_u16 (0x0001) ; const SKIP_BYTES : usize = usize :: repeat_u16 (0x00ff) ; let pair_sum : usize = (values & SKIP_BYTES) + ((values >> 8) & SKIP_BYTES) ; pair_sum . wrapping_mul (LSB_SHORTS) >> ((USIZE_SIZE - 2) * 8) }
}

macro_rules! char_count_general_case_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function char_count_general_case in module {}", module_path!());
    };
}

mkfn!{
    char_count_general_case_introspect!();
    fn char_count_general_case (s : & [u8]) -> usize { s . iter () . filter (| & & byte | ! super :: validations :: utf8_is_cont_byte (byte)) . count () }
}