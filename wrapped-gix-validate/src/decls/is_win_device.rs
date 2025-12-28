macro_rules! is_win_device {
    () => {
        fn is_win_device (input : & BStr) -> bool { let Some (in3) = input . get (.. 3) else { return false } ; if in3 . eq_ignore_ascii_case (b"AUX") && is_done_windows (input . get (3 ..)) { return true ; } if in3 . eq_ignore_ascii_case (b"NUL") && is_done_windows (input . get (3 ..)) { return true ; } if in3 . eq_ignore_ascii_case (b"PRN") && is_done_windows (input . get (3 ..)) { return true ; } if in3 . eq_ignore_ascii_case (b"COM") && input . get (3) . is_some_and (| n | * n >= b'1' && * n <= b'9') && is_done_windows (input . get (4 ..)) { return true ; } if in3 . eq_ignore_ascii_case (b"LPT") && input . get (3) . is_some_and (u8 :: is_ascii_digit) && is_done_windows (input . get (4 ..)) { return true ; } if in3 . eq_ignore_ascii_case (b"CON") && (is_done_windows (input . get (3 ..)) || (input . get (3 .. 6) . is_some_and (| n | n . eq_ignore_ascii_case (b"IN$")) && is_done_windows (input . get (6 ..))) || (input . get (3 .. 7) . is_some_and (| n | n . eq_ignore_ascii_case (b"OUT$")) && is_done_windows (input . get (7 ..)))) { return true ; } false }
    };
}

is_win_device!()