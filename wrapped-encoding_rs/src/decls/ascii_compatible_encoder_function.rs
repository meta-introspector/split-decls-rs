macro_rules! deps {
    () => {
        CopyAsciiResult!();
        ByteDestination!();
        NonAscii!();
        Space!();
        EncoderResult!();
        Unicode!();
    };
}

macro_rules! ascii_compatible_encoder_function {
    () => {
        deps!();
        macro_rules ! ascii_compatible_encoder_function { ($ bmp_body : block , $ astral_body : block , $ bmp : ident , $ astral : ident , $ slf : ident , $ source : ident , $ handle : ident , $ copy_ascii : ident , $ destination_check : ident , $ name : ident , $ input : ty , $ source_struct : ident , $ ascii_punctuation : expr) => (pub fn $ name (& mut $ slf , src : &$ input , dst : & mut [u8] , _last : bool) -> (EncoderResult , usize , usize) { let mut $ source = $ source_struct :: new (src) ; let mut dest = ByteDestination :: new (dst) ; 'outermost : loop { match $ source .$ copy_ascii (& mut dest) { CopyAsciiResult :: Stop (ret) => return ret , CopyAsciiResult :: GoOn ((mut non_ascii , mut $ handle)) => { 'middle : loop { let dest_again = match non_ascii { NonAscii :: BmpExclAscii ($ bmp) => { $ bmp_body } NonAscii :: Astral ($ astral) => { $ astral_body } } ; match $ source . check_available () { Space :: Full (src_consumed) => { return (EncoderResult :: InputEmpty , src_consumed , dest_again . written ()) ; } Space :: Available (source_handle) => { match dest_again .$ destination_check () { Space :: Full (dst_written) => { return (EncoderResult :: OutputFull , source_handle . consumed () , dst_written) ; } Space :: Available (mut destination_handle) => { let (mut c , unread_handle) = source_handle . read_enum () ; let source_again = unread_handle . commit () ; 'innermost : loop { let ascii = match c { Unicode :: NonAscii (non_ascii_again) => { non_ascii = non_ascii_again ; $ handle = destination_handle ; continue 'middle ; } Unicode :: Ascii (a) => a , } ; let dest_again_again = destination_handle . write_one (ascii) ; if $ ascii_punctuation && ascii < 60 { match source_again . check_available () { Space :: Full (src_consumed_again) => { return (EncoderResult :: InputEmpty , src_consumed_again , dest_again_again . written ()) ; } Space :: Available (source_handle_again) => { match dest_again_again .$ destination_check () { Space :: Full (dst_written_again) => { return (EncoderResult :: OutputFull , source_handle_again . consumed () , dst_written_again) ; } Space :: Available (destination_handle_again) => { { let (c_again , unread_handle_again) = source_handle_again . read_enum () ; unread_handle_again . commit () ; c = c_again ; destination_handle = destination_handle_again ; continue 'innermost ; } } } } } } continue 'outermost ; } } } } } } } } } }) ; }
    };
}

ascii_compatible_encoder_function!();