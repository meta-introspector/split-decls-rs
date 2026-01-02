mkuse!{use crate :: cmp :: Ordering ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: num :: bignum :: { Big32x40 as Big , Digit32 as Digit } ;}
mkuse!{use crate :: num :: flt2dec :: estimator :: estimate_scaling_factor ;}
mkuse!{use crate :: num :: flt2dec :: { Decoded , MAX_SIG_DIGITS , round_up } ;}
mkitem!{static POW10 : [Digit ; 10] = [1 , 10 , 100 , 1000 , 10000 , 100000 , 1000000 , 10000000 , 100000000 , 1000000000] ;}
mkitem!{static POW5TO16 : [Digit ; 2] = [0x86f26fc1 , 0x23] ;}
mkitem!{static POW5TO32 : [Digit ; 3] = [0x85acef81 , 0x2d6d415b , 0x4ee] ;}
mkitem!{static POW5TO64 : [Digit ; 5] = [0xbf6a1f01 , 0x6e38ed64 , 0xdaa797ed , 0xe93ff9f4 , 0x184f03] ;}
mkitem!{static POW5TO128 : [Digit ; 10] = [0x2e953e01 , 0x3df9909 , 0xf1538fd , 0x2374e42f , 0xd3cff5ec , 0xc404dc08 , 0xbccdb0da , 0xa6337f19 , 0xe91f2603 , 0x24e ,] ;}
mkitem!{static POW5TO256 : [Digit ; 19] = [0x982e7c01 , 0xbed3875b , 0xd8d99f72 , 0x12152f87 , 0x6bde50c6 , 0xcf4a6e70 , 0xd595d80f , 0x26b2716e , 0xadc666b0 , 0x1d153624 , 0x3c42d35a , 0x63ff540e , 0xcc5573c0 , 0x65f9ef17 , 0x55bc28f2 , 0x80dcc7f7 , 0xf46eeddc , 0x5fdcefce , 0x553f7 ,] ;}

macro_rules! mul_pow10_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mul_pow10 in module {}", module_path!());
    };
}

mkfn!{
    mul_pow10_introspect!();
    # [doc (hidden)] pub fn mul_pow10 (x : & mut Big , n : usize) -> & mut Big { debug_assert ! (n < 512) ; if n < 8 { return x . mul_small (POW10 [n & 7]) ; } if n & 7 != 0 { x . mul_small (POW10 [n & 7] >> (n & 7)) ; } if n & 8 != 0 { x . mul_small (POW10 [8] >> 8) ; } if n & 16 != 0 { x . mul_digits (& POW5TO16) ; } if n & 32 != 0 { x . mul_digits (& POW5TO32) ; } if n & 64 != 0 { x . mul_digits (& POW5TO64) ; } if n & 128 != 0 { x . mul_digits (& POW5TO128) ; } if n & 256 != 0 { x . mul_digits (& POW5TO256) ; } x . mul_pow2 (n) }
}

macro_rules! div_2pow10_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function div_2pow10 in module {}", module_path!());
    };
}

mkfn!{
    div_2pow10_introspect!();
    fn div_2pow10 (x : & mut Big , mut n : usize) -> & mut Big { let largest = POW10 . len () - 1 ; while n > largest { x . div_rem_small (POW10 [largest]) ; n -= largest ; } x . div_rem_small (POW10 [n] << 1) ; x }
}

macro_rules! div_rem_upto_16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function div_rem_upto_16 in module {}", module_path!());
    };
}

mkfn!{
    div_rem_upto_16_introspect!();
    fn div_rem_upto_16 < 'a > (x : & 'a mut Big , scale : & Big , scale2 : & Big , scale4 : & Big , scale8 : & Big ,) -> (u8 , & 'a mut Big) { let mut d = 0 ; if * x >= * scale8 { x . sub (scale8) ; d += 8 ; } if * x >= * scale4 { x . sub (scale4) ; d += 4 ; } if * x >= * scale2 { x . sub (scale2) ; d += 2 ; } if * x >= * scale { x . sub (scale) ; d += 1 ; } debug_assert ! (* x < * scale) ; (d , x) }
}

macro_rules! format_shortest_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function format_shortest in module {}", module_path!());
    };
}

mkfn!{
    format_shortest_introspect!();
    # [doc = " The shortest mode implementation for Dragon."] pub fn format_shortest < 'a > (d : & Decoded , buf : & 'a mut [MaybeUninit < u8 >] ,) -> (& 'a [u8] , i16) { assert ! (d . mant > 0) ; assert ! (d . minus > 0) ; assert ! (d . plus > 0) ; assert ! (d . mant . checked_add (d . plus) . is_some ()) ; assert ! (d . mant . checked_sub (d . minus) . is_some ()) ; assert ! (buf . len () >= MAX_SIG_DIGITS) ; let rounding = if d . inclusive { Ordering :: Greater } else { Ordering :: Equal } ; let mut k = estimate_scaling_factor (d . mant + d . plus , d . exp) ; let mut mant = Big :: from_u64 (d . mant) ; let mut minus = Big :: from_u64 (d . minus) ; let mut plus = Big :: from_u64 (d . plus) ; let mut scale = Big :: from_small (1) ; if d . exp < 0 { scale . mul_pow2 (- d . exp as usize) ; } else { mant . mul_pow2 (d . exp as usize) ; minus . mul_pow2 (d . exp as usize) ; plus . mul_pow2 (d . exp as usize) ; } if k >= 0 { mul_pow10 (& mut scale , k as usize) ; } else { mul_pow10 (& mut mant , - k as usize) ; mul_pow10 (& mut minus , - k as usize) ; mul_pow10 (& mut plus , - k as usize) ; } if scale . cmp (mant . clone () . add (& plus)) < rounding { k += 1 ; } else { mant . mul_small (10) ; minus . mul_small (10) ; plus . mul_small (10) ; } let mut scale2 = scale . clone () ; scale2 . mul_pow2 (1) ; let mut scale4 = scale . clone () ; scale4 . mul_pow2 (2) ; let mut scale8 = scale . clone () ; scale8 . mul_pow2 (3) ; let mut down ; let mut up ; let mut i = 0 ; loop { let (d , _) = div_rem_upto_16 (& mut mant , & scale , & scale2 , & scale4 , & scale8) ; debug_assert ! (d < 10) ; buf [i] = MaybeUninit :: new (b'0' + d) ; i += 1 ; down = mant . cmp (& minus) < rounding ; up = scale . cmp (mant . clone () . add (& plus)) < rounding ; if down || up { break ; } mant . mul_small (10) ; minus . mul_small (10) ; plus . mul_small (10) ; } if up && (! down || * mant . mul_pow2 (1) >= scale) { if let Some (c) = round_up (unsafe { buf [.. i] . assume_init_mut () }) { buf [i] = MaybeUninit :: new (c) ; i += 1 ; k += 1 ; } } (unsafe { buf [.. i] . assume_init_ref () } , k) }
}

macro_rules! format_exact_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function format_exact in module {}", module_path!());
    };
}

mkfn!{
    format_exact_introspect!();
    # [doc = " The exact and fixed mode implementation for Dragon."] pub fn format_exact < 'a > (d : & Decoded , buf : & 'a mut [MaybeUninit < u8 >] , limit : i16 ,) -> (& 'a [u8] , i16) { assert ! (d . mant > 0) ; assert ! (d . minus > 0) ; assert ! (d . plus > 0) ; assert ! (d . mant . checked_add (d . plus) . is_some ()) ; assert ! (d . mant . checked_sub (d . minus) . is_some ()) ; let mut k = estimate_scaling_factor (d . mant , d . exp) ; let mut mant = Big :: from_u64 (d . mant) ; let mut scale = Big :: from_small (1) ; if d . exp < 0 { scale . mul_pow2 (- d . exp as usize) ; } else { mant . mul_pow2 (d . exp as usize) ; } if k >= 0 { mul_pow10 (& mut scale , k as usize) ; } else { mul_pow10 (& mut mant , - k as usize) ; } if * div_2pow10 (& mut scale . clone () , buf . len ()) . add (& mant) >= scale { k += 1 ; } else { mant . mul_small (10) ; } let mut len = if k < limit { 0 } else if ((k as i32 - limit as i32) as usize) < buf . len () { (k - limit) as usize } else { buf . len () } ; if len > 0 { let mut scale2 = scale . clone () ; scale2 . mul_pow2 (1) ; let mut scale4 = scale . clone () ; scale4 . mul_pow2 (2) ; let mut scale8 = scale . clone () ; scale8 . mul_pow2 (3) ; for i in 0 .. len { if mant . is_zero () { for c in & mut buf [i .. len] { * c = MaybeUninit :: new (b'0') ; } return (unsafe { buf [.. len] . assume_init_ref () } , k) ; } let mut d = 0 ; if mant >= scale8 { mant . sub (& scale8) ; d += 8 ; } if mant >= scale4 { mant . sub (& scale4) ; d += 4 ; } if mant >= scale2 { mant . sub (& scale2) ; d += 2 ; } if mant >= scale { mant . sub (& scale) ; d += 1 ; } debug_assert ! (mant < scale) ; debug_assert ! (d < 10) ; buf [i] = MaybeUninit :: new (b'0' + d) ; mant . mul_small (10) ; } } let order = mant . cmp (scale . mul_small (5)) ; if order == Ordering :: Greater || (order == Ordering :: Equal && len > 0 && unsafe { buf [len - 1] . assume_init () } & 1 == 1) { if let Some (c) = round_up (unsafe { buf [.. len] . assume_init_mut () }) { k += 1 ; if k > limit && len < buf . len () { buf [len] = MaybeUninit :: new (c) ; len += 1 ; } } } (unsafe { buf [.. len] . assume_init_ref () } , k) }
}