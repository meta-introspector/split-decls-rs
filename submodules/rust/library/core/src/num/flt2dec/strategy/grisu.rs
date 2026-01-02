mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: num :: diy_float :: Fp ;}
mkuse!{use crate :: num :: flt2dec :: { Decoded , MAX_SIG_DIGITS , round_up } ;}
mkitem!{# [doc (hidden)] pub const ALPHA : i16 = - 60 ;}
mkitem!{# [doc (hidden)] pub const GAMMA : i16 = - 32 ;}
mkitem!{# [doc (hidden)] pub static CACHED_POW10 : [(u64 , i16 , i16) ; 81] = [(0xe61acf033d1a45df , - 1087 , - 308) , (0xab70fe17c79ac6ca , - 1060 , - 300) , (0xff77b1fcbebcdc4f , - 1034 , - 292) , (0xbe5691ef416bd60c , - 1007 , - 284) , (0x8dd01fad907ffc3c , - 980 , - 276) , (0xd3515c2831559a83 , - 954 , - 268) , (0x9d71ac8fada6c9b5 , - 927 , - 260) , (0xea9c227723ee8bcb , - 901 , - 252) , (0xaecc49914078536d , - 874 , - 244) , (0x823c12795db6ce57 , - 847 , - 236) , (0xc21094364dfb5637 , - 821 , - 228) , (0x9096ea6f3848984f , - 794 , - 220) , (0xd77485cb25823ac7 , - 768 , - 212) , (0xa086cfcd97bf97f4 , - 741 , - 204) , (0xef340a98172aace5 , - 715 , - 196) , (0xb23867fb2a35b28e , - 688 , - 188) , (0x84c8d4dfd2c63f3b , - 661 , - 180) , (0xc5dd44271ad3cdba , - 635 , - 172) , (0x936b9fcebb25c996 , - 608 , - 164) , (0xdbac6c247d62a584 , - 582 , - 156) , (0xa3ab66580d5fdaf6 , - 555 , - 148) , (0xf3e2f893dec3f126 , - 529 , - 140) , (0xb5b5ada8aaff80b8 , - 502 , - 132) , (0x87625f056c7c4a8b , - 475 , - 124) , (0xc9bcff6034c13053 , - 449 , - 116) , (0x964e858c91ba2655 , - 422 , - 108) , (0xdff9772470297ebd , - 396 , - 100) , (0xa6dfbd9fb8e5b88f , - 369 , - 92) , (0xf8a95fcf88747d94 , - 343 , - 84) , (0xb94470938fa89bcf , - 316 , - 76) , (0x8a08f0f8bf0f156b , - 289 , - 68) , (0xcdb02555653131b6 , - 263 , - 60) , (0x993fe2c6d07b7fac , - 236 , - 52) , (0xe45c10c42a2b3b06 , - 210 , - 44) , (0xaa242499697392d3 , - 183 , - 36) , (0xfd87b5f28300ca0e , - 157 , - 28) , (0xbce5086492111aeb , - 130 , - 20) , (0x8cbccc096f5088cc , - 103 , - 12) , (0xd1b71758e219652c , - 77 , - 4) , (0x9c40000000000000 , - 50 , 4) , (0xe8d4a51000000000 , - 24 , 12) , (0xad78ebc5ac620000 , 3 , 20) , (0x813f3978f8940984 , 30 , 28) , (0xc097ce7bc90715b3 , 56 , 36) , (0x8f7e32ce7bea5c70 , 83 , 44) , (0xd5d238a4abe98068 , 109 , 52) , (0x9f4f2726179a2245 , 136 , 60) , (0xed63a231d4c4fb27 , 162 , 68) , (0xb0de65388cc8ada8 , 189 , 76) , (0x83c7088e1aab65db , 216 , 84) , (0xc45d1df942711d9a , 242 , 92) , (0x924d692ca61be758 , 269 , 100) , (0xda01ee641a708dea , 295 , 108) , (0xa26da3999aef774a , 322 , 116) , (0xf209787bb47d6b85 , 348 , 124) , (0xb454e4a179dd1877 , 375 , 132) , (0x865b86925b9bc5c2 , 402 , 140) , (0xc83553c5c8965d3d , 428 , 148) , (0x952ab45cfa97a0b3 , 455 , 156) , (0xde469fbd99a05fe3 , 481 , 164) , (0xa59bc234db398c25 , 508 , 172) , (0xf6c69a72a3989f5c , 534 , 180) , (0xb7dcbf5354e9bece , 561 , 188) , (0x88fcf317f22241e2 , 588 , 196) , (0xcc20ce9bd35c78a5 , 614 , 204) , (0x98165af37b2153df , 641 , 212) , (0xe2a0b5dc971f303a , 667 , 220) , (0xa8d9d1535ce3b396 , 694 , 228) , (0xfb9b7cd9a4a7443c , 720 , 236) , (0xbb764c4ca7a44410 , 747 , 244) , (0x8bab8eefb6409c1a , 774 , 252) , (0xd01fef10a657842c , 800 , 260) , (0x9b10a4e5e9913129 , 827 , 268) , (0xe7109bfba19c0c9d , 853 , 276) , (0xac2820d9623bf429 , 880 , 284) , (0x80444b5e7aa7cf85 , 907 , 292) , (0xbf21e44003acdd2d , 933 , 300) , (0x8e679c2f5e44ff8f , 960 , 308) , (0xd433179d9c8cb841 , 986 , 316) , (0x9e19db92b4e31ba9 , 1013 , 324) , (0xeb96bf6ebadf77d9 , 1039 , 332) ,] ;}
mkitem!{# [doc (hidden)] pub const CACHED_POW10_FIRST_E : i16 = - 1087 ;}
mkitem!{# [doc (hidden)] pub const CACHED_POW10_LAST_E : i16 = 1039 ;}

macro_rules! cached_power_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cached_power in module {}", module_path!());
    };
}

mkfn!{
    cached_power_introspect!();
    # [doc (hidden)] pub fn cached_power (alpha : i16 , gamma : i16) -> (i16 , Fp) { let offset = CACHED_POW10_FIRST_E as i32 ; let range = (CACHED_POW10 . len () as i32) - 1 ; let domain = (CACHED_POW10_LAST_E - CACHED_POW10_FIRST_E) as i32 ; let idx = ((gamma as i32) - offset) * range / domain ; let (f , e , k) = CACHED_POW10 [idx as usize] ; debug_assert ! (alpha <= e && e <= gamma) ; (k , Fp { f , e }) }
}

macro_rules! max_pow10_no_more_than_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function max_pow10_no_more_than in module {}", module_path!());
    };
}

mkfn!{
    max_pow10_no_more_than_introspect!();
    # [doc = " Given `x > 0`, returns `(k, 10^k)` such that `10^k <= x < 10^(k+1)`."] # [doc (hidden)] pub fn max_pow10_no_more_than (x : u32) -> (u8 , u32) { debug_assert ! (x > 0) ; const X9 : u32 = 10_0000_0000 ; const X8 : u32 = 1_0000_0000 ; const X7 : u32 = 1000_0000 ; const X6 : u32 = 100_0000 ; const X5 : u32 = 10_0000 ; const X4 : u32 = 1_0000 ; const X3 : u32 = 1000 ; const X2 : u32 = 100 ; const X1 : u32 = 10 ; if x < X4 { if x < X2 { if x < X1 { (0 , 1) } else { (1 , X1) } } else { if x < X3 { (2 , X2) } else { (3 , X3) } } } else { if x < X6 { if x < X5 { (4 , X4) } else { (5 , X5) } } else if x < X8 { if x < X7 { (6 , X6) } else { (7 , X7) } } else { if x < X9 { (8 , X8) } else { (9 , X9) } } } }
}

macro_rules! format_shortest_opt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function format_shortest_opt in module {}", module_path!());
    };
}

mkfn!{
    format_shortest_opt_introspect!();
    # [doc = " The shortest mode implementation for Grisu."] # [doc = ""] # [doc = " It returns `None` when it would return an inexact representation otherwise."] pub fn format_shortest_opt < 'a > (d : & Decoded , buf : & 'a mut [MaybeUninit < u8 >] ,) -> Option < (& 'a [u8] , i16) > { assert ! (d . mant > 0) ; assert ! (d . minus > 0) ; assert ! (d . plus > 0) ; assert ! (d . mant . checked_add (d . plus) . is_some ()) ; assert ! (d . mant . checked_sub (d . minus) . is_some ()) ; assert ! (buf . len () >= MAX_SIG_DIGITS) ; assert ! (d . mant + d . plus < (1 << 61)) ; let plus = Fp { f : d . mant + d . plus , e : d . exp } . normalize () ; let minus = Fp { f : d . mant - d . minus , e : d . exp } . normalize_to (plus . e) ; let v = Fp { f : d . mant , e : d . exp } . normalize_to (plus . e) ; let (minusk , cached) = cached_power (ALPHA - plus . e - 64 , GAMMA - plus . e - 64) ; let plus = plus . mul (cached) ; let minus = minus . mul (cached) ; let v = v . mul (cached) ; debug_assert_eq ! (plus . e , minus . e) ; debug_assert_eq ! (plus . e , v . e) ; let plus1 = plus . f + 1 ; let minus1 = minus . f - 1 ; let e = - plus . e as usize ; let plus1int = (plus1 >> e) as u32 ; let plus1frac = plus1 & ((1 << e) - 1) ; let (max_kappa , max_ten_kappa) = max_pow10_no_more_than (plus1int) ; let mut i = 0 ; let exp = max_kappa as i16 - minusk + 1 ; let delta1 = plus1 - minus1 ; let delta1frac = delta1 & ((1 << e) - 1) ; let mut ten_kappa = max_ten_kappa ; let mut remainder = plus1int ; loop { let q = remainder / ten_kappa ; let r = remainder % ten_kappa ; debug_assert ! (q < 10) ; buf [i] = MaybeUninit :: new (b'0' + q as u8) ; i += 1 ; let plus1rem = ((r as u64) << e) + plus1frac ; if plus1rem < delta1 { let ten_kappa = (ten_kappa as u64) << e ; return round_and_weed (unsafe { buf [.. i] . assume_init_mut () } , exp , plus1rem , delta1 , plus1 - v . f , ten_kappa , 1 ,) ; } if i > max_kappa as usize { debug_assert_eq ! (ten_kappa , 1) ; break ; } ten_kappa /= 10 ; remainder = r ; } let mut remainder = plus1frac ; let mut threshold = delta1frac ; let mut ulp = 1 ; loop { remainder *= 10 ; threshold *= 10 ; ulp *= 10 ; let q = remainder >> e ; let r = remainder & ((1 << e) - 1) ; debug_assert ! (q < 10) ; buf [i] = MaybeUninit :: new (b'0' + q as u8) ; i += 1 ; if r < threshold { let ten_kappa = 1 << e ; return round_and_weed (unsafe { buf [.. i] . assume_init_mut () } , exp , r , threshold , (plus1 - v . f) * ulp , ten_kappa , ulp ,) ; } remainder = r ; } fn round_and_weed (buf : & mut [u8] , exp : i16 , remainder : u64 , threshold : u64 , plus1v : u64 , ten_kappa : u64 , ulp : u64 ,) -> Option < (& [u8] , i16) > { assert ! (! buf . is_empty ()) ; let plus1v_down = plus1v + ulp ; let plus1v_up = plus1v - ulp ; let mut plus1w = remainder ; { let last = buf . last_mut () . unwrap () ; while plus1w < plus1v_up && threshold - plus1w >= ten_kappa && (plus1w + ten_kappa < plus1v_up || plus1v_up - plus1w >= plus1w + ten_kappa - plus1v_up) { * last -= 1 ; debug_assert ! (* last > b'0') ; plus1w += ten_kappa ; } } if plus1w < plus1v_down && threshold - plus1w >= ten_kappa && (plus1w + ten_kappa < plus1v_down || plus1v_down - plus1w >= plus1w + ten_kappa - plus1v_down) { return None ; } if 2 * ulp <= plus1w && plus1w <= threshold - 4 * ulp { Some ((buf , exp)) } else { None } } }
}

macro_rules! format_shortest_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function format_shortest in module {}", module_path!());
    };
}

mkfn!{
    format_shortest_introspect!();
    # [doc = " The shortest mode implementation for Grisu with Dragon fallback."] # [doc = ""] # [doc = " This should be used for most cases."] pub fn format_shortest < 'a > (d : & Decoded , buf : & 'a mut [MaybeUninit < u8 >] ,) -> (& 'a [u8] , i16) { use crate :: num :: flt2dec :: strategy :: dragon :: format_shortest as fallback ; match format_shortest_opt (d , unsafe { & mut * (buf as * mut _) }) { Some (ret) => ret , None => fallback (d , buf) , } }
}

macro_rules! format_exact_opt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function format_exact_opt in module {}", module_path!());
    };
}

mkfn!{
    format_exact_opt_introspect!();
    # [doc = " The exact and fixed mode implementation for Grisu."] # [doc = ""] # [doc = " It returns `None` when it would return an inexact representation otherwise."] pub fn format_exact_opt < 'a > (d : & Decoded , buf : & 'a mut [MaybeUninit < u8 >] , limit : i16 ,) -> Option < (& 'a [u8] , i16) > { assert ! (d . mant > 0) ; assert ! (d . mant < (1 << 61)) ; assert ! (! buf . is_empty ()) ; let v = Fp { f : d . mant , e : d . exp } . normalize () ; let (minusk , cached) = cached_power (ALPHA - v . e - 64 , GAMMA - v . e - 64) ; let v = v . mul (cached) ; let e = - v . e as usize ; let vint = (v . f >> e) as u32 ; let vfrac = v . f & ((1 << e) - 1) ; let requested_digits = buf . len () ; const POW10_UP_TO_9 : [u32 ; 10] = [1 , 10 , 100 , 1000 , 10_000 , 100_000 , 1_000_000 , 10_000_000 , 100_000_000 , 1_000_000_000] ; if (vfrac == 0) && ((requested_digits >= 11) || (vint < POW10_UP_TO_9 [requested_digits - 1])) { return None ; } let mut err = 1 ; let (max_kappa , max_ten_kappa) = max_pow10_no_more_than (vint) ; let mut i = 0 ; let exp = max_kappa as i16 - minusk + 1 ; let len = if exp <= limit { return unsafe { possibly_round (buf , 0 , exp , limit , v . f / 10 , (max_ten_kappa as u64) << e , err << e) } ; } else if ((exp as i32 - limit as i32) as usize) < buf . len () { (exp - limit) as usize } else { buf . len () } ; debug_assert ! (len > 0) ; let mut kappa = max_kappa as i16 ; let mut ten_kappa = max_ten_kappa ; let mut remainder = vint ; loop { let q = remainder / ten_kappa ; let r = remainder % ten_kappa ; debug_assert ! (q < 10) ; buf [i] = MaybeUninit :: new (b'0' + q as u8) ; i += 1 ; if i == len { let vrem = ((r as u64) << e) + vfrac ; return unsafe { possibly_round (buf , len , exp , limit , vrem , (ten_kappa as u64) << e , err << e) } ; } if i > max_kappa as usize { debug_assert_eq ! (ten_kappa , 1) ; debug_assert_eq ! (kappa , 0) ; break ; } kappa -= 1 ; ten_kappa /= 10 ; remainder = r ; } let mut remainder = vfrac ; let maxerr = 1 << (e - 1) ; while err < maxerr { remainder *= 10 ; err *= 10 ; let q = remainder >> e ; let r = remainder & ((1 << e) - 1) ; debug_assert ! (q < 10) ; buf [i] = MaybeUninit :: new (b'0' + q as u8) ; i += 1 ; if i == len { return unsafe { possibly_round (buf , len , exp , limit , r , 1 << e , err) } ; } remainder = r ; } return None ; unsafe fn possibly_round (buf : & mut [MaybeUninit < u8 >] , mut len : usize , mut exp : i16 , limit : i16 , remainder : u64 , ten_kappa : u64 , ulp : u64 ,) -> Option < (& [u8] , i16) > { debug_assert ! (remainder < ten_kappa) ; if ulp >= ten_kappa { return None ; } if ten_kappa - ulp <= ulp { return None ; } if ten_kappa - remainder > remainder && ten_kappa - 2 * remainder >= 2 * ulp { return Some ((unsafe { buf [.. len] . assume_init_ref () } , exp)) ; } if remainder > ulp && ten_kappa - (remainder - ulp) <= remainder - ulp { if let Some (c) = round_up (unsafe { buf [.. len] . assume_init_mut () }) { exp += 1 ; if exp > limit && len < buf . len () { buf [len] = MaybeUninit :: new (c) ; len += 1 ; } } return Some ((unsafe { buf [.. len] . assume_init_ref () } , exp)) ; } None } }
}

macro_rules! format_exact_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function format_exact in module {}", module_path!());
    };
}

mkfn!{
    format_exact_introspect!();
    # [doc = " The exact and fixed mode implementation for Grisu with Dragon fallback."] # [doc = ""] # [doc = " This should be used for most cases."] pub fn format_exact < 'a > (d : & Decoded , buf : & 'a mut [MaybeUninit < u8 >] , limit : i16 ,) -> (& 'a [u8] , i16) { use crate :: num :: flt2dec :: strategy :: dragon :: format_exact as fallback ; match format_exact_opt (d , unsafe { & mut * (buf as * mut _) } , limit) { Some (ret) => ret , None => fallback (d , buf , limit) , } }
}