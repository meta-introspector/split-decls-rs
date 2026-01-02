mkitem!{# [doc = " This array stores the [integer square roots]("] # [doc = " https://en.wikipedia.org/wiki/Integer_square_root) and remainders of each"] # [doc = " [`u8`](prim@u8) value. For example, `U8_ISQRT_WITH_REMAINDER[17]` will be"] # [doc = " `(4, 1)` because the integer square root of 17 is 4 and because 17 is 1"] # [doc = " higher than 4 squared."] const U8_ISQRT_WITH_REMAINDER : [(u8 , u8) ; 256] = { let mut result = [(0 , 0) ; 256] ; let mut n : usize = 0 ; let mut isqrt_n : usize = 0 ; while n < result . len () { result [n] = (isqrt_n as u8 , (n - isqrt_n . pow (2)) as u8) ; n += 1 ; if n == (isqrt_n + 1) . pow (2) { isqrt_n += 1 ; } } result } ;}

macro_rules! u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u8 in module {}", module_path!());
    };
}

mkfn!{
    u8_introspect!();
    # [doc = " Returns the [integer square root]("] # [doc = " https://en.wikipedia.org/wiki/Integer_square_root) of any [`u8`](prim@u8)"] # [doc = " input."] # [must_use = "this returns the result of the operation, \
              without modifying the original"] # [inline] pub (super) const fn u8 (n : u8) -> u8 { U8_ISQRT_WITH_REMAINDER [n as usize] . 0 }
}
mkitem!{# [doc = " Generates an `i*` function that returns the [integer square root]("] # [doc = " https://en.wikipedia.org/wiki/Integer_square_root) of any **nonnegative**"] # [doc = " input of a specific signed integer type."] macro_rules ! signed_fn { ($ SignedT : ident , $ UnsignedT : ident) => { # [doc = " Returns the [integer square root]("] # [doc = " https://en.wikipedia.org/wiki/Integer_square_root) of any"] # [doc = " **nonnegative**"] # [doc = concat ! ("[`" , stringify ! ($ SignedT) , "`](prim@" , stringify ! ($ SignedT) , ")")] # [doc = " input."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This results in undefined behavior when the input is negative."] # [must_use = "this returns the result of the operation, \
                      without modifying the original"] # [inline] pub (super) const unsafe fn $ SignedT (n : $ SignedT) -> $ SignedT { debug_assert ! (n >= 0 , "Negative input inside `isqrt`.") ; $ UnsignedT (n as $ UnsignedT) as $ SignedT } } ; }}
mkitem!{signed_fn ! (i8 , u8) ;}
mkitem!{signed_fn ! (i16 , u16) ;}
mkitem!{signed_fn ! (i32 , u32) ;}
mkitem!{signed_fn ! (i64 , u64) ;}
mkitem!{signed_fn ! (i128 , u128) ;}
mkitem!{# [doc = " Generates a `u*` function that returns the [integer square root]("] # [doc = " https://en.wikipedia.org/wiki/Integer_square_root) of any input of"] # [doc = " a specific unsigned integer type."] macro_rules ! unsigned_fn { ($ UnsignedT : ident , $ HalfBitsT : ident , $ stages : ident) => { # [doc = " Returns the [integer square root]("] # [doc = " https://en.wikipedia.org/wiki/Integer_square_root) of any"] # [doc = concat ! ("[`" , stringify ! ($ UnsignedT) , "`](prim@" , stringify ! ($ UnsignedT) , ")")] # [doc = " input."] # [must_use = "this returns the result of the operation, \
                      without modifying the original"] # [inline] pub (super) const fn $ UnsignedT (mut n : $ UnsignedT) -> $ UnsignedT { if n <= <$ HalfBitsT >:: MAX as $ UnsignedT { $ HalfBitsT (n as $ HalfBitsT) as $ UnsignedT } else { const EVEN_MAKING_BITMASK : u32 = ! 1 ; let normalization_shift = n . leading_zeros () & EVEN_MAKING_BITMASK ; n <<= normalization_shift ; let s = $ stages (n) ; let denormalization_shift = normalization_shift >> 1 ; s >> denormalization_shift } } } ; }}
mkitem!{# [doc = " Generates the first stage of the computation after normalization."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `$n` must be nonzero."] macro_rules ! first_stage { ($ original_bits : literal , $ n : ident) => { { debug_assert ! ($ n != 0 , "`$n` is  zero in `first_stage!`.") ; const N_SHIFT : u32 = $ original_bits - 8 ; let n = $ n >> N_SHIFT ; let (s , r) = U8_ISQRT_WITH_REMAINDER [n as usize] ; unsafe { crate :: hint :: assert_unchecked (s != 0) } ; (s , r) } } ; }}
mkitem!{# [doc = " Generates a middle stage of the computation."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `$s` must be nonzero."] macro_rules ! middle_stage { ($ original_bits : literal , $ ty : ty , $ n : ident , $ s : ident , $ r : ident) => { { debug_assert ! ($ s != 0 , "`$s` is  zero in `middle_stage!`.") ; const N_SHIFT : u32 = $ original_bits - <$ ty >:: BITS ; let n = ($ n >> N_SHIFT) as $ ty ; const HALF_BITS : u32 = <$ ty >:: BITS >> 1 ; const QUARTER_BITS : u32 = <$ ty >:: BITS >> 2 ; const LOWER_HALF_1_BITS : $ ty = (1 << HALF_BITS) - 1 ; const LOWEST_QUARTER_1_BITS : $ ty = (1 << QUARTER_BITS) - 1 ; let lo = n & LOWER_HALF_1_BITS ; let numerator = (($ r as $ ty) << QUARTER_BITS) | (lo >> QUARTER_BITS) ; let denominator = ($ s as $ ty) << 1 ; let q = numerator / denominator ; let u = numerator % denominator ; let mut s = ($ s << QUARTER_BITS) as $ ty + q ; let (mut r , overflow) = ((u << QUARTER_BITS) | (lo & LOWEST_QUARTER_1_BITS)) . overflowing_sub (q * q) ; if overflow { r = r . wrapping_add (2 * s - 1) ; s -= 1 ; } unsafe { crate :: hint :: assert_unchecked (s != 0) } ; (s , r) } } ; }}
mkitem!{# [doc = " Generates the last stage of the computation before denormalization."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `$s` must be nonzero."] macro_rules ! last_stage { ($ ty : ty , $ n : ident , $ s : ident , $ r : ident) => { { debug_assert ! ($ s != 0 , "`$s` is  zero in `last_stage!`.") ; const HALF_BITS : u32 = <$ ty >:: BITS >> 1 ; const QUARTER_BITS : u32 = <$ ty >:: BITS >> 2 ; const LOWER_HALF_1_BITS : $ ty = (1 << HALF_BITS) - 1 ; let lo = $ n & LOWER_HALF_1_BITS ; let numerator = (($ r as $ ty) << QUARTER_BITS) | (lo >> QUARTER_BITS) ; let denominator = ($ s as $ ty) << 1 ; let q = numerator / denominator ; let mut s = ($ s << QUARTER_BITS) as $ ty + q ; let (s_squared , overflow) = s . overflowing_mul (s) ; if overflow || s_squared > $ n { s -= 1 ; } s } } ; }}

macro_rules! u16_stages_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u16_stages in module {}", module_path!());
    };
}

mkfn!{
    u16_stages_introspect!();
    # [doc = " Takes the normalized [`u16`](prim@u16) input and gets its normalized"] # [doc = " [integer square root](https://en.wikipedia.org/wiki/Integer_square_root)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `n` must be nonzero."] # [inline] const fn u16_stages (n : u16) -> u16 { let (s , r) = first_stage ! (16 , n) ; last_stage ! (u16 , n , s , r) }
}

macro_rules! u32_stages_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u32_stages in module {}", module_path!());
    };
}

mkfn!{
    u32_stages_introspect!();
    # [doc = " Takes the normalized [`u32`](prim@u32) input and gets its normalized"] # [doc = " [integer square root](https://en.wikipedia.org/wiki/Integer_square_root)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `n` must be nonzero."] # [inline] const fn u32_stages (n : u32) -> u32 { let (s , r) = first_stage ! (32 , n) ; let (s , r) = middle_stage ! (32 , u16 , n , s , r) ; last_stage ! (u32 , n , s , r) }
}

macro_rules! u64_stages_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u64_stages in module {}", module_path!());
    };
}

mkfn!{
    u64_stages_introspect!();
    # [doc = " Takes the normalized [`u64`](prim@u64) input and gets its normalized"] # [doc = " [integer square root](https://en.wikipedia.org/wiki/Integer_square_root)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `n` must be nonzero."] # [inline] const fn u64_stages (n : u64) -> u64 { let (s , r) = first_stage ! (64 , n) ; let (s , r) = middle_stage ! (64 , u16 , n , s , r) ; let (s , r) = middle_stage ! (64 , u32 , n , s , r) ; last_stage ! (u64 , n , s , r) }
}

macro_rules! u128_stages_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u128_stages in module {}", module_path!());
    };
}

mkfn!{
    u128_stages_introspect!();
    # [doc = " Takes the normalized [`u128`](prim@u128) input and gets its normalized"] # [doc = " [integer square root](https://en.wikipedia.org/wiki/Integer_square_root)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `n` must be nonzero."] # [inline] const fn u128_stages (n : u128) -> u128 { let (s , r) = first_stage ! (128 , n) ; let (s , r) = middle_stage ! (128 , u16 , n , s , r) ; let (s , r) = middle_stage ! (128 , u32 , n , s , r) ; let (s , r) = middle_stage ! (128 , u64 , n , s , r) ; last_stage ! (u128 , n , s , r) }
}
mkitem!{unsigned_fn ! (u16 , u8 , u16_stages) ;}
mkitem!{unsigned_fn ! (u32 , u16 , u32_stages) ;}
mkitem!{unsigned_fn ! (u64 , u32 , u64_stages) ;}
mkitem!{unsigned_fn ! (u128 , u64 , u128_stages) ;}

macro_rules! panic_for_negative_argument_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_for_negative_argument in module {}", module_path!());
    };
}

mkfn!{
    panic_for_negative_argument_introspect!();
    # [doc = " Instantiate this panic logic once, rather than for all the isqrt methods"] # [doc = " on every single primitive type."] # [cold] # [track_caller] pub (super) const fn panic_for_negative_argument () -> ! { panic ! ("argument of integer square root cannot be negative") }
}