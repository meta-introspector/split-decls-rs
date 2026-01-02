mkuse!{pub use self :: decoder :: { DecodableFloat , Decoded , FullDecoded , decode } ;}
mkuse!{use super :: fmt :: { Formatted , Part } ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkmod!{decoder, { 
                getname!(decoder);
                getsrc!(decoder);
                getpath!(decoder);
                get_deps!(decoder);
                get_crates!(decoder);
                mkinclude!(decoder);
                 
            }}
mkmod!{estimator, { 
                getname!(estimator);
                getsrc!(estimator);
                getpath!(estimator);
                get_deps!(estimator);
                get_crates!(estimator);
                mkinclude!(estimator);
                 
            }}
mkmod!{strategy, { 
                getname!(strategy);
                getsrc!(strategy);
                getpath!(strategy);
                get_deps!(strategy);
                get_crates!(strategy);
                mkinclude!(strategy);
                mkmod!{dragon, { 
                getname!(dragon);
                getsrc!(dragon);
                getpath!(dragon);
                get_deps!(dragon);
                get_crates!(dragon);
                mkinclude!(dragon);
                 
            }}
mkmod!{grisu, { 
                getname!(grisu);
                getsrc!(grisu);
                getpath!(grisu);
                get_deps!(grisu);
                get_crates!(grisu);
                mkinclude!(grisu);
                 
            }} 
            }}
mkitem!{# [doc = " The minimum size of buffer necessary for the shortest mode."] # [doc = ""] # [doc = " It is a bit non-trivial to derive, but this is one plus the maximal number of"] # [doc = " significant decimal digits from formatting algorithms with the shortest result."] # [doc = " The exact formula is `ceil(# bits in mantissa * log_10 2 + 1)`."] pub const MAX_SIG_DIGITS : usize = 17 ;}

macro_rules! round_up_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function round_up in module {}", module_path!());
    };
}

mkfn!{
    round_up_introspect!();
    # [doc = " When `d` contains decimal digits, increase the last digit and propagate carry."] # [doc = " Returns a next digit when it causes the length to change."] # [doc (hidden)] pub fn round_up (d : & mut [u8]) -> Option < u8 > { match d . iter () . rposition (| & c | c != b'9') { Some (i) => { d [i] += 1 ; d [i + 1 ..] . fill (b'0') ; None } None if d . is_empty () => { Some (b'1') } None => { d [0] = b'1' ; d [1 ..] . fill (b'0') ; Some (b'0') } } }
}

macro_rules! digits_to_dec_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function digits_to_dec_str in module {}", module_path!());
    };
}

mkfn!{
    digits_to_dec_str_introspect!();
    # [doc = " Formats given decimal digits `0.<...buf...> * 10^exp` into the decimal form"] # [doc = " with at least given number of fractional digits. The result is stored to"] # [doc = " the supplied parts array and a slice of written parts is returned."] # [doc = ""] # [doc = " `frac_digits` can be less than the number of actual fractional digits in `buf`;"] # [doc = " it will be ignored and full digits will be printed. It is only used to print"] # [doc = " additional zeroes after rendered digits. Thus `frac_digits` of 0 means that"] # [doc = " it will only print given digits and nothing else."] fn digits_to_dec_str < 'a > (buf : & 'a [u8] , exp : i16 , frac_digits : usize , parts : & 'a mut [MaybeUninit < Part < 'a > >] ,) -> & 'a [Part < 'a >] { assert ! (! buf . is_empty ()) ; assert ! (buf [0] > b'0') ; assert ! (parts . len () >= 4) ; if exp <= 0 { let minus_exp = - (exp as i32) as usize ; parts [0] = MaybeUninit :: new (Part :: Copy (b"0.")) ; parts [1] = MaybeUninit :: new (Part :: Zero (minus_exp)) ; parts [2] = MaybeUninit :: new (Part :: Copy (buf)) ; if frac_digits > buf . len () && frac_digits - buf . len () > minus_exp { parts [3] = MaybeUninit :: new (Part :: Zero ((frac_digits - buf . len ()) - minus_exp)) ; unsafe { parts [.. 4] . assume_init_ref () } } else { unsafe { parts [.. 3] . assume_init_ref () } } } else { let exp = exp as usize ; if exp < buf . len () { parts [0] = MaybeUninit :: new (Part :: Copy (& buf [.. exp])) ; parts [1] = MaybeUninit :: new (Part :: Copy (b".")) ; parts [2] = MaybeUninit :: new (Part :: Copy (& buf [exp ..])) ; if frac_digits > buf . len () - exp { parts [3] = MaybeUninit :: new (Part :: Zero (frac_digits - (buf . len () - exp))) ; unsafe { parts [.. 4] . assume_init_ref () } } else { unsafe { parts [.. 3] . assume_init_ref () } } } else { parts [0] = MaybeUninit :: new (Part :: Copy (buf)) ; parts [1] = MaybeUninit :: new (Part :: Zero (exp - buf . len ())) ; if frac_digits > 0 { parts [2] = MaybeUninit :: new (Part :: Copy (b".")) ; parts [3] = MaybeUninit :: new (Part :: Zero (frac_digits)) ; unsafe { parts [.. 4] . assume_init_ref () } } else { unsafe { parts [.. 2] . assume_init_ref () } } } } }
}

macro_rules! digits_to_exp_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function digits_to_exp_str in module {}", module_path!());
    };
}

mkfn!{
    digits_to_exp_str_introspect!();
    # [doc = " Formats the given decimal digits `0.<...buf...> * 10^exp` into the exponential"] # [doc = " form with at least the given number of significant digits. When `upper` is `true`,"] # [doc = " the exponent will be prefixed by `E`; otherwise that's `e`. The result is"] # [doc = " stored to the supplied parts array and a slice of written parts is returned."] # [doc = ""] # [doc = " `min_digits` can be less than the number of actual significant digits in `buf`;"] # [doc = " it will be ignored and full digits will be printed. It is only used to print"] # [doc = " additional zeroes after rendered digits. Thus, `min_digits == 0` means that"] # [doc = " it will only print the given digits and nothing else."] fn digits_to_exp_str < 'a > (buf : & 'a [u8] , exp : i16 , min_ndigits : usize , upper : bool , parts : & 'a mut [MaybeUninit < Part < 'a > >] ,) -> & 'a [Part < 'a >] { assert ! (! buf . is_empty ()) ; assert ! (buf [0] > b'0') ; assert ! (parts . len () >= 6) ; let mut n = 0 ; parts [n] = MaybeUninit :: new (Part :: Copy (& buf [.. 1])) ; n += 1 ; if buf . len () > 1 || min_ndigits > 1 { parts [n] = MaybeUninit :: new (Part :: Copy (b".")) ; parts [n + 1] = MaybeUninit :: new (Part :: Copy (& buf [1 ..])) ; n += 2 ; if min_ndigits > buf . len () { parts [n] = MaybeUninit :: new (Part :: Zero (min_ndigits - buf . len ())) ; n += 1 ; } } let exp = exp as i32 - 1 ; if exp < 0 { parts [n] = MaybeUninit :: new (Part :: Copy (if upper { b"E-" } else { b"e-" })) ; parts [n + 1] = MaybeUninit :: new (Part :: Num (- exp as u16)) ; } else { parts [n] = MaybeUninit :: new (Part :: Copy (if upper { b"E" } else { b"e" })) ; parts [n + 1] = MaybeUninit :: new (Part :: Num (exp as u16)) ; } unsafe { parts [.. n + 2] . assume_init_ref () } }
}
mkitem!{mkenum!{# [doc = " Sign formatting options."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum Sign { # [doc = " Prints `-` for any negative value."] Minus , # [doc = " Prints `-` for any negative value, or `+` otherwise."] MinusPlus , }}}

macro_rules! determine_sign_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function determine_sign in module {}", module_path!());
    };
}

mkfn!{
    determine_sign_introspect!();
    # [doc = " Returns the static byte string corresponding to the sign to be formatted."] # [doc = " It can be either `\"\"`, `\"+\"` or `\"-\"`."] fn determine_sign (sign : Sign , decoded : & FullDecoded , negative : bool) -> & 'static str { match (* decoded , sign) { (FullDecoded :: Nan , _) => "" , (_ , Sign :: Minus) => { if negative { "-" } else { "" } } (_ , Sign :: MinusPlus) => { if negative { "-" } else { "+" } } } }
}

macro_rules! to_shortest_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_shortest_str in module {}", module_path!());
    };
}

mkfn!{
    to_shortest_str_introspect!();
    # [doc = " Formats the given floating point number into the decimal form with at least"] # [doc = " given number of fractional digits. The result is stored to the supplied parts"] # [doc = " array while utilizing given byte buffer as a scratch. `upper` is currently"] # [doc = " unused but left for the future decision to change the case of non-finite values,"] # [doc = " i.e., `inf` and `nan`. The first part to be rendered is always a `Part::Sign`"] # [doc = " (which can be an empty string if no sign is rendered)."] # [doc = ""] # [doc = " `format_shortest` should be the underlying digit-generation function."] # [doc = " It should return the part of the buffer that it initialized."] # [doc = " You probably would want `strategy::grisu::format_shortest` for this."] # [doc = ""] # [doc = " `frac_digits` can be less than the number of actual fractional digits in `v`;"] # [doc = " it will be ignored and full digits will be printed. It is only used to print"] # [doc = " additional zeroes after rendered digits. Thus `frac_digits` of 0 means that"] # [doc = " it will only print given digits and nothing else."] # [doc = ""] # [doc = " The byte buffer should be at least `MAX_SIG_DIGITS` bytes long."] # [doc = " There should be at least 4 parts available, due to the worst case like"] # [doc = " `[+][0.][0000][2][0000]` with `frac_digits = 10`."] pub fn to_shortest_str < 'a , T , F > (mut format_shortest : F , v : T , sign : Sign , frac_digits : usize , buf : & 'a mut [MaybeUninit < u8 >] , parts : & 'a mut [MaybeUninit < Part < 'a > >] ,) -> Formatted < 'a > where T : DecodableFloat , F : FnMut (& Decoded , & 'a mut [MaybeUninit < u8 >]) -> (& 'a [u8] , i16) , { assert ! (parts . len () >= 4) ; assert ! (buf . len () >= MAX_SIG_DIGITS) ; let (negative , full_decoded) = decode (v) ; let sign = determine_sign (sign , & full_decoded , negative) ; match full_decoded { FullDecoded :: Nan => { parts [0] = MaybeUninit :: new (Part :: Copy (b"NaN")) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } } } FullDecoded :: Infinite => { parts [0] = MaybeUninit :: new (Part :: Copy (b"inf")) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } } } FullDecoded :: Zero => { if frac_digits > 0 { parts [0] = MaybeUninit :: new (Part :: Copy (b"0.")) ; parts [1] = MaybeUninit :: new (Part :: Zero (frac_digits)) ; Formatted { sign , parts : unsafe { parts [.. 2] . assume_init_ref () } , } } else { parts [0] = MaybeUninit :: new (Part :: Copy (b"0")) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } , } } } FullDecoded :: Finite (ref decoded) => { let (buf , exp) = format_shortest (decoded , buf) ; Formatted { sign , parts : digits_to_dec_str (buf , exp , frac_digits , parts) } } } }
}

macro_rules! to_shortest_exp_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_shortest_exp_str in module {}", module_path!());
    };
}

mkfn!{
    to_shortest_exp_str_introspect!();
    # [doc = " Formats the given floating point number into the decimal form or"] # [doc = " the exponential form, depending on the resulting exponent. The result is"] # [doc = " stored to the supplied parts array while utilizing given byte buffer"] # [doc = " as a scratch. `upper` is used to determine the case of non-finite values"] # [doc = " (`inf` and `nan`) or the case of the exponent prefix (`e` or `E`)."] # [doc = " The first part to be rendered is always a `Part::Sign` (which can be"] # [doc = " an empty string if no sign is rendered)."] # [doc = ""] # [doc = " `format_shortest` should be the underlying digit-generation function."] # [doc = " It should return the part of the buffer that it initialized."] # [doc = " You probably would want `strategy::grisu::format_shortest` for this."] # [doc = ""] # [doc = " The `dec_bounds` is a tuple `(lo, hi)` such that the number is formatted"] # [doc = " as decimal only when `10^lo <= V < 10^hi`. Note that this is the *apparent* `V`"] # [doc = " instead of the actual `v`! Thus any printed exponent in the exponential form"] # [doc = " cannot be in this range, avoiding any confusion."] # [doc = ""] # [doc = " The byte buffer should be at least `MAX_SIG_DIGITS` bytes long."] # [doc = " There should be at least 6 parts available, due to the worst case like"] # [doc = " `[+][1][.][2345][e][-][6]`."] pub fn to_shortest_exp_str < 'a , T , F > (mut format_shortest : F , v : T , sign : Sign , dec_bounds : (i16 , i16) , upper : bool , buf : & 'a mut [MaybeUninit < u8 >] , parts : & 'a mut [MaybeUninit < Part < 'a > >] ,) -> Formatted < 'a > where T : DecodableFloat , F : FnMut (& Decoded , & 'a mut [MaybeUninit < u8 >]) -> (& 'a [u8] , i16) , { assert ! (parts . len () >= 6) ; assert ! (buf . len () >= MAX_SIG_DIGITS) ; assert ! (dec_bounds . 0 <= dec_bounds . 1) ; let (negative , full_decoded) = decode (v) ; let sign = determine_sign (sign , & full_decoded , negative) ; match full_decoded { FullDecoded :: Nan => { parts [0] = MaybeUninit :: new (Part :: Copy (b"NaN")) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } } } FullDecoded :: Infinite => { parts [0] = MaybeUninit :: new (Part :: Copy (b"inf")) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } } } FullDecoded :: Zero => { parts [0] = if dec_bounds . 0 <= 0 && 0 < dec_bounds . 1 { MaybeUninit :: new (Part :: Copy (b"0")) } else { MaybeUninit :: new (Part :: Copy (if upper { b"0E0" } else { b"0e0" })) } ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } } } FullDecoded :: Finite (ref decoded) => { let (buf , exp) = format_shortest (decoded , buf) ; let vis_exp = exp as i32 - 1 ; let parts = if dec_bounds . 0 as i32 <= vis_exp && vis_exp < dec_bounds . 1 as i32 { digits_to_dec_str (buf , exp , 0 , parts) } else { digits_to_exp_str (buf , exp , 0 , upper , parts) } ; Formatted { sign , parts } } } }
}

macro_rules! estimate_max_buf_len_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function estimate_max_buf_len in module {}", module_path!());
    };
}

mkfn!{
    estimate_max_buf_len_introspect!();
    # [doc = " Returns a rather crude approximation (upper bound) for the maximum buffer size"] # [doc = " calculated from the given decoded exponent."] # [doc = ""] # [doc = " The exact limit is:"] # [doc = ""] # [doc = " - when `exp < 0`, the maximum length is `ceil(log_10 (5^-exp * (2^64 - 1)))`."] # [doc = " - when `exp >= 0`, the maximum length is `ceil(log_10 (2^exp * (2^64 - 1)))`."] # [doc = ""] # [doc = " `ceil(log_10 (x^exp * (2^64 - 1)))` is less than `ceil(log_10 (2^64 - 1)) +"] # [doc = " ceil(exp * log_10 x)`, which is in turn less than `20 + (1 + exp * log_10 x)`."] # [doc = " We use the facts that `log_10 2 < 5/16` and `log_10 5 < 12/16`, which is"] # [doc = " enough for our purposes."] # [doc = ""] # [doc = " Why do we need this? `format_exact` functions will fill the entire buffer"] # [doc = " unless limited by the last digit restriction, but it is possible that"] # [doc = " the number of digits requested is ridiculously large (say, 30,000 digits)."] # [doc = " The vast majority of buffer will be filled with zeroes, so we don't want to"] # [doc = " allocate all the buffer beforehand. Consequently, for any given arguments,"] # [doc = " 826 bytes of buffer should be sufficient for `f64`. Compare this with"] # [doc = " the actual number for the worst case: 770 bytes (when `exp = -1074`)."] fn estimate_max_buf_len (exp : i16) -> usize { 21 + ((if exp < 0 { - 12 } else { 5 } * exp as i32) as usize >> 4) }
}

macro_rules! to_exact_exp_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_exact_exp_str in module {}", module_path!());
    };
}

mkfn!{
    to_exact_exp_str_introspect!();
    # [doc = " Formats given floating point number into the exponential form with"] # [doc = " exactly given number of significant digits. The result is stored to"] # [doc = " the supplied parts array while utilizing given byte buffer as a scratch."] # [doc = " `upper` is used to determine the case of the exponent prefix (`e` or `E`)."] # [doc = " The first part to be rendered is always a `Part::Sign` (which can be"] # [doc = " an empty string if no sign is rendered)."] # [doc = ""] # [doc = " `format_exact` should be the underlying digit-generation function."] # [doc = " It should return the part of the buffer that it initialized."] # [doc = " You probably would want `strategy::grisu::format_exact` for this."] # [doc = ""] # [doc = " The byte buffer should be at least `ndigits` bytes long unless `ndigits` is"] # [doc = " so large that only the fixed number of digits will be ever written."] # [doc = " (The tipping point for `f64` is about 800, so 1000 bytes should be enough.)"] # [doc = " There should be at least 6 parts available, due to the worst case like"] # [doc = " `[+][1][.][2345][e][-][6]`."] pub fn to_exact_exp_str < 'a , T , F > (mut format_exact : F , v : T , sign : Sign , ndigits : usize , upper : bool , buf : & 'a mut [MaybeUninit < u8 >] , parts : & 'a mut [MaybeUninit < Part < 'a > >] ,) -> Formatted < 'a > where T : DecodableFloat , F : FnMut (& Decoded , & 'a mut [MaybeUninit < u8 >] , i16) -> (& 'a [u8] , i16) , { assert ! (parts . len () >= 6) ; assert ! (ndigits > 0) ; let (negative , full_decoded) = decode (v) ; let sign = determine_sign (sign , & full_decoded , negative) ; match full_decoded { FullDecoded :: Nan => { parts [0] = MaybeUninit :: new (Part :: Copy (b"NaN")) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } } } FullDecoded :: Infinite => { parts [0] = MaybeUninit :: new (Part :: Copy (b"inf")) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } } } FullDecoded :: Zero => { if ndigits > 1 { parts [0] = MaybeUninit :: new (Part :: Copy (b"0.")) ; parts [1] = MaybeUninit :: new (Part :: Zero (ndigits - 1)) ; parts [2] = MaybeUninit :: new (Part :: Copy (if upper { b"E0" } else { b"e0" })) ; Formatted { sign , parts : unsafe { parts [.. 3] . assume_init_ref () } , } } else { parts [0] = MaybeUninit :: new (Part :: Copy (if upper { b"0E0" } else { b"0e0" })) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } , } } } FullDecoded :: Finite (ref decoded) => { let maxlen = estimate_max_buf_len (decoded . exp) ; assert ! (buf . len () >= ndigits || buf . len () >= maxlen) ; let trunc = if ndigits < maxlen { ndigits } else { maxlen } ; let (buf , exp) = format_exact (decoded , & mut buf [.. trunc] , i16 :: MIN) ; Formatted { sign , parts : digits_to_exp_str (buf , exp , ndigits , upper , parts) } } } }
}

macro_rules! to_exact_fixed_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_exact_fixed_str in module {}", module_path!());
    };
}

mkfn!{
    to_exact_fixed_str_introspect!();
    # [doc = " Formats given floating point number into the decimal form with exactly"] # [doc = " given number of fractional digits. The result is stored to the supplied parts"] # [doc = " array while utilizing given byte buffer as a scratch. `upper` is currently"] # [doc = " unused but left for the future decision to change the case of non-finite values,"] # [doc = " i.e., `inf` and `nan`. The first part to be rendered is always a `Part::Sign`"] # [doc = " (which can be an empty string if no sign is rendered)."] # [doc = ""] # [doc = " `format_exact` should be the underlying digit-generation function."] # [doc = " It should return the part of the buffer that it initialized."] # [doc = " You probably would want `strategy::grisu::format_exact` for this."] # [doc = ""] # [doc = " The byte buffer should be enough for the output unless `frac_digits` is"] # [doc = " so large that only the fixed number of digits will be ever written."] # [doc = " (The tipping point for `f64` is about 800, and 1000 bytes should be enough.)"] # [doc = " There should be at least 4 parts available, due to the worst case like"] # [doc = " `[+][0.][0000][2][0000]` with `frac_digits = 10`."] pub fn to_exact_fixed_str < 'a , T , F > (mut format_exact : F , v : T , sign : Sign , frac_digits : usize , buf : & 'a mut [MaybeUninit < u8 >] , parts : & 'a mut [MaybeUninit < Part < 'a > >] ,) -> Formatted < 'a > where T : DecodableFloat , F : FnMut (& Decoded , & 'a mut [MaybeUninit < u8 >] , i16) -> (& 'a [u8] , i16) , { assert ! (parts . len () >= 4) ; let (negative , full_decoded) = decode (v) ; let sign = determine_sign (sign , & full_decoded , negative) ; match full_decoded { FullDecoded :: Nan => { parts [0] = MaybeUninit :: new (Part :: Copy (b"NaN")) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } } } FullDecoded :: Infinite => { parts [0] = MaybeUninit :: new (Part :: Copy (b"inf")) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } } } FullDecoded :: Zero => { if frac_digits > 0 { parts [0] = MaybeUninit :: new (Part :: Copy (b"0.")) ; parts [1] = MaybeUninit :: new (Part :: Zero (frac_digits)) ; Formatted { sign , parts : unsafe { parts [.. 2] . assume_init_ref () } , } } else { parts [0] = MaybeUninit :: new (Part :: Copy (b"0")) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } , } } } FullDecoded :: Finite (ref decoded) => { let maxlen = estimate_max_buf_len (decoded . exp) ; assert ! (buf . len () >= maxlen) ; let limit = if frac_digits < 0x8000 { - (frac_digits as i16) } else { i16 :: MIN } ; let (buf , exp) = format_exact (decoded , & mut buf [.. maxlen] , limit) ; if exp <= limit { debug_assert_eq ! (buf . len () , 0) ; if frac_digits > 0 { parts [0] = MaybeUninit :: new (Part :: Copy (b"0.")) ; parts [1] = MaybeUninit :: new (Part :: Zero (frac_digits)) ; Formatted { sign , parts : unsafe { parts [.. 2] . assume_init_ref () } , } } else { parts [0] = MaybeUninit :: new (Part :: Copy (b"0")) ; Formatted { sign , parts : unsafe { parts [.. 1] . assume_init_ref () } , } } } else { Formatted { sign , parts : digits_to_dec_str (buf , exp , frac_digits , parts) } } } } }
}