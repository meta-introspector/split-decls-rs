macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! hex_check_sse_with_case {
    () => {
        deps!();
        # [doc = " # Safety"] # [doc = " Check if a byte slice is valid on given check_case."] # [target_feature (enable = "sse4.1")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] pub unsafe fn hex_check_sse_with_case (mut src : & [u8] , check_case : CheckCase) -> bool { let ascii_zero = _mm_set1_epi8 ((b'0' - 1) as i8) ; let ascii_nine = _mm_set1_epi8 ((b'9' + 1) as i8) ; let ascii_ua = _mm_set1_epi8 ((b'A' - 1) as i8) ; let ascii_uf = _mm_set1_epi8 ((b'F' + 1) as i8) ; let ascii_la = _mm_set1_epi8 ((b'a' - 1) as i8) ; let ascii_lf = _mm_set1_epi8 ((b'f' + 1) as i8) ; while src . len () >= 16 { let unchecked = _mm_loadu_si128 (src . as_ptr () as * const _) ; let gt0 = _mm_cmpgt_epi8 (unchecked , ascii_zero) ; let lt9 = _mm_cmplt_epi8 (unchecked , ascii_nine) ; let valid_digit = _mm_and_si128 (gt0 , lt9) ; let (valid_la_lf , valid_ua_uf) = match check_case { CheckCase :: None => { let gtua = _mm_cmpgt_epi8 (unchecked , ascii_ua) ; let ltuf = _mm_cmplt_epi8 (unchecked , ascii_uf) ; let gtla = _mm_cmpgt_epi8 (unchecked , ascii_la) ; let ltlf = _mm_cmplt_epi8 (unchecked , ascii_lf) ; (Some (_mm_and_si128 (gtla , ltlf)) , Some (_mm_and_si128 (gtua , ltuf)) ,) } CheckCase :: Lower => { let gtla = _mm_cmpgt_epi8 (unchecked , ascii_la) ; let ltlf = _mm_cmplt_epi8 (unchecked , ascii_lf) ; (Some (_mm_and_si128 (gtla , ltlf)) , None) } CheckCase :: Upper => { let gtua = _mm_cmpgt_epi8 (unchecked , ascii_ua) ; let ltuf = _mm_cmplt_epi8 (unchecked , ascii_uf) ; (None , Some (_mm_and_si128 (gtua , ltuf))) } } ; let valid_letter = match (valid_la_lf , valid_ua_uf) { (Some (valid_lower) , Some (valid_upper)) => _mm_or_si128 (valid_lower , valid_upper) , (Some (valid_lower) , None) => valid_lower , (None , Some (valid_upper)) => valid_upper , _ => unreachable ! () , } ; let ret = _mm_movemask_epi8 (_mm_or_si128 (valid_digit , valid_letter)) ; if ret != T_MASK { return false ; } src = & src [16 ..] ; } hex_check_fallback_with_case (src , check_case) }
    };
}

hex_check_sse_with_case!();