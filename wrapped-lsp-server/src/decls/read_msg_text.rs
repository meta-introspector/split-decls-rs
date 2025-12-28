macro_rules! read_msg_text {
    () => {
        fn read_msg_text (inp : & mut dyn BufRead) -> io :: Result < Option < String > > { let mut size = None ; let mut buf = String :: new () ; loop { buf . clear () ; if inp . read_line (& mut buf) ? == 0 { return Ok (None) ; } if ! buf . ends_with ("\r\n") { return Err (invalid_data ! ("malformed header: {:?}" , buf)) ; } let buf = & buf [.. buf . len () - 2] ; if buf . is_empty () { break ; } let mut parts = buf . splitn (2 , ": ") ; let header_name = parts . next () . unwrap () ; let header_value = parts . next () . ok_or_else (| | invalid_data ! ("malformed header: {:?}" , buf)) ? ; if header_name . eq_ignore_ascii_case ("Content-Length") { size = Some (header_value . parse :: < usize > () . map_err (invalid_data) ?) ; } } let size : usize = size . ok_or_else (| | invalid_data ! ("no Content-Length")) ? ; let mut buf = buf . into_bytes () ; buf . resize (size , 0) ; inp . read_exact (& mut buf) ? ; let buf = String :: from_utf8 (buf) . map_err (invalid_data) ? ; log :: debug ! ("< {buf}") ; Ok (Some (buf)) }
    };
}

read_msg_text!();