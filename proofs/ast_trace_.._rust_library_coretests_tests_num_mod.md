# AST Trace: ../rust/library/coretests/tests/num/mod.rs

Generated 172 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use core::fmt::Debug;
use core::num::{IntErrorKind, ParseIntError, TryFromIntError, can_not_overflow};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use core::ops::{Add, Div, Mul, Rem, Sub};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=10 | LINES=41

```rust
use core::str::FromStr;

#[macro_use]
mod int_macros;

mod i128;
mod i16;
mod i32;
mod i64;
mod i8;

#[macro_use]
mod uint_macros;

mod u128;
mod u16;
mod u32;
mod u64;
mod u8;

mod bignum;
mod const_from;
mod dec2flt;
mod float_iter_sum_identity;
mod flt2dec;
mod ieee754;
mod int_log;
mod int_sqrt;
mod midpoint;
mod nan;
mod niche_types;
mod ops;
mod wrapping;

/// Adds the attribute to all items in the block.
macro_rules! cfg_block {
    ($(#[$attr:meta]{$($it:item)*})*) => {$($(
        #[$attr]
        $it
    )*)*}
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=12 | LINES=11

```rust
/// Groups items that assume the pointer width is either 16/32/64, and has to be altered if
/// support for larger/smaller pointer widths are added in the future.
macro_rules! assume_usize_width {
    {$($it:item)*} => {#[cfg(not(any(
        target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64")))]
           compile_error!("The current tests of try_from on usize/isize assume that \
                           the pointer width is either 16, 32, or 64");
                    $($it)*
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=ldexp_f16 | COMPLEXITY=2 | LINES=6

```rust
/// Return `a * 2^b`.
#[cfg(target_has_reliable_f16)]
fn ldexp_f16(a: f16, b: i32) -> f16 {
    ldexp_f64(a as f64, b) as f16
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=ldexp_f32 | COMPLEXITY=2 | LINES=5

```rust
/// Return `a * 2^b`.
fn ldexp_f32(a: f32, b: i32) -> f32 {
    ldexp_f64(a as f64, b) as f32
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=ldexp_f64 | COMPLEXITY=12 | LINES=10

```rust
/// Return `a * 2^b`.
fn ldexp_f64(a: f64, b: i32) -> f64 {
    unsafe extern "C" {
        fn ldexp(x: f64, n: i32) -> f64;
    }
    // SAFETY: assuming a correct `ldexp` has been supplied, the given arguments cannot possibly
    // cause undefined behavior
    unsafe { ldexp(a, b) }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=test_num | COMPLEXITY=4 | LINES=19

```rust
/// Helper function for testing numeric operations
pub fn test_num<T>(ten: T, two: T)
where
    T: PartialEq
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Debug
        + Copy,
{
    assert_eq!(ten.add(two), ten + two);
    assert_eq!(ten.sub(two), ten - two);
    assert_eq!(ten.mul(two), ten * two);
    assert_eq!(ten.div(two), ten / two);
    assert_eq!(ten.rem(two), ten % two);
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=test_parse | COMPLEXITY=4 | LINES=9

```rust
/// Helper function for asserting number parsing returns a specific error
fn test_parse<T>(num_str: &str, expected: Result<T, IntErrorKind>)
where
    T: FromStr<Err = ParseIntError>,
    Result<T, IntErrorKind>: PartialEq + Debug,
{
    assert_eq!(num_str.parse::<T>().map_err(|e| e.kind().clone()), expected)
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=from_str_issue7588 | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn from_str_issue7588() {
    let u: Option<u8> = u8::from_str_radix("1000", 10).ok();
    assert_eq!(u, None);
    let s: Option<i16> = i16::from_str_radix("80000", 10).ok();
    assert_eq!(s, None);
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=test_int_from_str_overflow | COMPLEXITY=4 | LINES=27

```rust
#[test]
fn test_int_from_str_overflow() {
    test_parse::<i8>("127", Ok(127));
    test_parse::<i8>("128", Err(IntErrorKind::PosOverflow));

    test_parse::<i8>("-128", Ok(-128));
    test_parse::<i8>("-129", Err(IntErrorKind::NegOverflow));

    test_parse::<i16>("32767", Ok(32_767));
    test_parse::<i16>("32768", Err(IntErrorKind::PosOverflow));

    test_parse::<i16>("-32768", Ok(-32_768));
    test_parse::<i16>("-32769", Err(IntErrorKind::NegOverflow));

    test_parse::<i32>("2147483647", Ok(2_147_483_647));
    test_parse::<i32>("2147483648", Err(IntErrorKind::PosOverflow));

    test_parse::<i32>("-2147483648", Ok(-2_147_483_648));
    test_parse::<i32>("-2147483649", Err(IntErrorKind::NegOverflow));

    test_parse::<i64>("9223372036854775807", Ok(9_223_372_036_854_775_807));
    test_parse::<i64>("9223372036854775808", Err(IntErrorKind::PosOverflow));

    test_parse::<i64>("-9223372036854775808", Ok(-9_223_372_036_854_775_808));
    test_parse::<i64>("-9223372036854775809", Err(IntErrorKind::NegOverflow));
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=test_can_not_overflow | COMPLEXITY=30 | LINES=69

```rust
#[test]
fn test_can_not_overflow() {
    fn can_overflow<T>(radix: u32, input: &str) -> bool
    where
        T: std::convert::TryFrom<i8>,
    {
        !can_not_overflow::<T>(radix, T::try_from(-1_i8).is_ok(), input.as_bytes())
    }

    // Positive tests:
    assert!(!can_overflow::<i8>(16, "F"));
    assert!(!can_overflow::<u8>(16, "FF"));

    assert!(!can_overflow::<i8>(10, "9"));
    assert!(!can_overflow::<u8>(10, "99"));

    // Negative tests:

    // Not currently in std lib (issue: #27728)
    fn format_radix<T>(mut x: T, radix: T) -> String
    where
        T: std::ops::Rem<Output = T>,
        T: std::ops::Div<Output = T>,
        T: std::cmp::PartialEq,
        T: std::default::Default,
        T: Copy,
        T: Default,
        u32: TryFrom<T>,
    {
        let mut result = vec![];

        loop {
            let m = x % radix;
            x = x / radix;
            result.push(
                std::char::from_digit(m.try_into().ok().unwrap(), radix.try_into().ok().unwrap())
                    .unwrap(),
            );
            if x == T::default() {
                break;
            }
        }
        result.into_iter().rev().collect()
    }

    macro_rules! check {
        ($($t:ty)*) => ($(
        for base in 2..=36 {
            let num = (<$t>::MAX as u128) + 1;

           // Calculate the string length for the smallest overflowing number:
           let max_len_string = format_radix(num, base as u128);
           // Ensure that string length is deemed to potentially overflow:
           assert!(can_overflow::<$t>(base, &max_len_string));
        }
        )*)
    }

    check! { i8 i16 i32 i64 i128 isize usize u8 u16 u32 u64 }

    // Check u128 separately:
    for base in 2..=36 {
        let num = <u128>::MAX;
        let max_len_string = format_radix(num, base as u128);
        // base 16 fits perfectly for u128 and won't overflow:
        assert_eq!(can_overflow::<u128>(base, &max_len_string), base != 16);
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=test_leading_plus | COMPLEXITY=2 | LINES=6

```rust
#[test]
fn test_leading_plus() {
    test_parse::<u8>("+127", Ok(127));
    test_parse::<i64>("+9223372036854775807", Ok(9223372036854775807));
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=test_invalid | COMPLEXITY=3 | LINES=12

```rust
#[test]
fn test_invalid() {
    test_parse::<i8>("--129", Err(IntErrorKind::InvalidDigit));
    test_parse::<i8>("++129", Err(IntErrorKind::InvalidDigit));
    test_parse::<u8>("Съешь", Err(IntErrorKind::InvalidDigit));
    test_parse::<u8>("123Hello", Err(IntErrorKind::InvalidDigit));
    test_parse::<i8>("--", Err(IntErrorKind::InvalidDigit));
    test_parse::<i8>("-", Err(IntErrorKind::InvalidDigit));
    test_parse::<i8>("+", Err(IntErrorKind::InvalidDigit));
    test_parse::<u8>("-1", Err(IntErrorKind::InvalidDigit));
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=test_empty | COMPLEXITY=2 | LINES=5

```rust
#[test]
fn test_empty() {
    test_parse::<u8>("", Err(IntErrorKind::Empty));
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=test_infallible_try_from_int_error | COMPLEXITY=3 | LINES=7

```rust
#[test]
fn test_infallible_try_from_int_error() {
    let func = |x: i8| -> Result<i32, TryFromIntError> { Ok(x.try_into()?) };

    assert!(func(0).is_ok());
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=9 | LINES=10

```rust
const _TEST_CONST_PARSE: () = {
    let Ok(-0x8000) = i16::from_str_radix("-8000", 16) else { panic!() };
    let Ok(12345) = u64::from_str_radix("12345", 10) else { panic!() };
    if let Err(e) = i8::from_str_radix("+", 10) {
        let IntErrorKind::InvalidDigit = e.kind() else { panic!() };
    } else {
        panic!()
    }
};
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=$fn_name | COMPLEXITY=12 | LINES=23

```rust
macro_rules! test_impl_from {
    ($fn_name:ident, bool, $target: ty) => {
        #[test]
        fn $fn_name() {
            let one: $target = 1;
            let zero: $target = 0;
            assert_eq!(one, <$target>::from(true));
            assert_eq!(zero, <$target>::from(false));
        }
    };
    ($fn_name: ident, $Small: ty, $Large: ty) => {
        #[test]
        fn $fn_name() {
            let small_max = <$Small>::MAX;
            let small_min = <$Small>::MIN;
            let large_max: $Large = small_max.into();
            let large_min: $Large = small_min.into();
            assert_eq!(large_max as $Small, small_max);
            assert_eq!(large_min as $Small, small_min);
        }
    };
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
// Unsigned -> Unsigned
test_impl_from! { test_u8u16, u8, u16 }
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u8u32, u8, u32 }
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u8u64, u8, u64 }
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u8usize, u8, usize }
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u16u32, u16, u32 }
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u16u64, u16, u64 }
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u32u64, u32, u64 }
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
// Signed -> Signed
test_impl_from! { test_i8i16, i8, i16 }
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_i8i32, i8, i32 }
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_i8i64, i8, i64 }
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_i8isize, i8, isize }
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_i16i32, i16, i32 }
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_i16i64, i16, i64 }
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_i32i64, i32, i64 }
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
// Unsigned -> Signed
test_impl_from! { test_u8i16, u8, i16 }
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u8i32, u8, i32 }
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u8i64, u8, i64 }
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u16i32, u16, i32 }
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u16i64, u16, i64 }
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u32i64, u32, i64 }
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
// Bool -> Integer
test_impl_from! { test_boolu8, bool, u8 }
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_boolu16, bool, u16 }
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_boolu32, bool, u32 }
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_boolu64, bool, u64 }
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_boolu128, bool, u128 }
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_booli8, bool, i8 }
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_booli16, bool, i16 }
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_booli32, bool, i32 }
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_booli64, bool, i64 }
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_booli128, bool, i128 }
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
// Signed -> Float
test_impl_from! { test_i8f32, i8, f32 }
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_i8f64, i8, f64 }
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_i16f32, i16, f32 }
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_i16f64, i16, f64 }
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_i32f64, i32, f64 }
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
// Unsigned -> Float
test_impl_from! { test_u8f32, u8, f32 }
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u8f64, u8, f64 }
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u16f32, u16, f32 }
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u16f64, u16, f64 }
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_from! { test_u32f64, u32, f64 }
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=FUNCTION | NAME=test_f32f64 | COMPLEXITY=4 | LINES=41

```rust
// Float -> Float
#[test]
fn test_f32f64() {
    let max: f64 = f32::MAX.into();
    assert_eq!(max as f32, f32::MAX);
    assert!(max.is_normal());

    let min: f64 = f32::MIN.into();
    assert_eq!(min as f32, f32::MIN);
    assert!(min.is_normal());

    let min_positive: f64 = f32::MIN_POSITIVE.into();
    assert_eq!(min_positive as f32, f32::MIN_POSITIVE);
    assert!(min_positive.is_normal());

    let epsilon: f64 = f32::EPSILON.into();
    assert_eq!(epsilon as f32, f32::EPSILON);
    assert!(epsilon.is_normal());

    let zero: f64 = (0.0f32).into();
    assert_eq!(zero as f32, 0.0f32);
    assert!(zero.is_sign_positive());

    let neg_zero: f64 = (-0.0f32).into();
    assert_eq!(neg_zero as f32, -0.0f32);
    assert!(neg_zero.is_sign_negative());

    let infinity: f64 = f32::INFINITY.into();
    assert_eq!(infinity as f32, f32::INFINITY);
    assert!(infinity.is_infinite());
    assert!(infinity.is_sign_positive());

    let neg_infinity: f64 = f32::NEG_INFINITY.into();
    assert_eq!(neg_infinity as f32, f32::NEG_INFINITY);
    assert!(neg_infinity.is_infinite());
    assert!(neg_infinity.is_sign_negative());

    let nan: f64 = f32::NAN.into();
    assert!(nan.is_nan());
}
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=FUNCTION | NAME=$fn_name | COMPLEXITY=10 | LINES=15

```rust
/// Conversions where the full width of $source can be represented as $target
macro_rules! test_impl_try_from_always_ok {
    ($fn_name:ident, $source:ty, $target: ty) => {
        #[test]
        fn $fn_name() {
            let max = <$source>::MAX;
            let min = <$source>::MIN;
            let zero: $source = 0;
            assert_eq!(<$target as TryFrom<$source>>::try_from(max).unwrap(), max as $target);
            assert_eq!(<$target as TryFrom<$source>>::try_from(min).unwrap(), min as $target);
            assert_eq!(<$target as TryFrom<$source>>::try_from(zero).unwrap(), zero as $target);
        }
    };
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_always_ok! { test_try_u8u8, u8, u8 }
```

## Block 62
**Metadata**: AST_ID=62 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u8u16, u8, u16 }
```

## Block 63
**Metadata**: AST_ID=63 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u8u32, u8, u32 }
```

## Block 64
**Metadata**: AST_ID=64 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u8u64, u8, u64 }
```

## Block 65
**Metadata**: AST_ID=65 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u8u128, u8, u128 }
```

## Block 66
**Metadata**: AST_ID=66 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u8i16, u8, i16 }
```

## Block 67
**Metadata**: AST_ID=67 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u8i32, u8, i32 }
```

## Block 68
**Metadata**: AST_ID=68 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u8i64, u8, i64 }
```

## Block 69
**Metadata**: AST_ID=69 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u8i128, u8, i128 }
```

## Block 70
**Metadata**: AST_ID=70 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_always_ok! { test_try_u16u16, u16, u16 }
```

## Block 71
**Metadata**: AST_ID=71 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u16u32, u16, u32 }
```

## Block 72
**Metadata**: AST_ID=72 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u16u64, u16, u64 }
```

## Block 73
**Metadata**: AST_ID=73 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u16u128, u16, u128 }
```

## Block 74
**Metadata**: AST_ID=74 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u16i32, u16, i32 }
```

## Block 75
**Metadata**: AST_ID=75 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u16i64, u16, i64 }
```

## Block 76
**Metadata**: AST_ID=76 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u16i128, u16, i128 }
```

## Block 77
**Metadata**: AST_ID=77 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_always_ok! { test_try_u32u32, u32, u32 }
```

## Block 78
**Metadata**: AST_ID=78 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u32u64, u32, u64 }
```

## Block 79
**Metadata**: AST_ID=79 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u32u128, u32, u128 }
```

## Block 80
**Metadata**: AST_ID=80 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u32i64, u32, i64 }
```

## Block 81
**Metadata**: AST_ID=81 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u32i128, u32, i128 }
```

## Block 82
**Metadata**: AST_ID=82 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_always_ok! { test_try_u64u64, u64, u64 }
```

## Block 83
**Metadata**: AST_ID=83 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u64u128, u64, u128 }
```

## Block 84
**Metadata**: AST_ID=84 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_u64i128, u64, i128 }
```

## Block 85
**Metadata**: AST_ID=85 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_always_ok! { test_try_u128u128, u128, u128 }
```

## Block 86
**Metadata**: AST_ID=86 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_always_ok! { test_try_i8i8, i8, i8 }
```

## Block 87
**Metadata**: AST_ID=87 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_i8i16, i8, i16 }
```

## Block 88
**Metadata**: AST_ID=88 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_i8i32, i8, i32 }
```

## Block 89
**Metadata**: AST_ID=89 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_i8i64, i8, i64 }
```

## Block 90
**Metadata**: AST_ID=90 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_i8i128, i8, i128 }
```

## Block 91
**Metadata**: AST_ID=91 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_always_ok! { test_try_i16i16, i16, i16 }
```

## Block 92
**Metadata**: AST_ID=92 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_i16i32, i16, i32 }
```

## Block 93
**Metadata**: AST_ID=93 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_i16i64, i16, i64 }
```

## Block 94
**Metadata**: AST_ID=94 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_i16i128, i16, i128 }
```

## Block 95
**Metadata**: AST_ID=95 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_always_ok! { test_try_i32i32, i32, i32 }
```

## Block 96
**Metadata**: AST_ID=96 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_i32i64, i32, i64 }
```

## Block 97
**Metadata**: AST_ID=97 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_i32i128, i32, i128 }
```

## Block 98
**Metadata**: AST_ID=98 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_always_ok! { test_try_i64i64, i64, i64 }
```

## Block 99
**Metadata**: AST_ID=99 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_i64i128, i64, i128 }
```

## Block 100
**Metadata**: AST_ID=100 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_always_ok! { test_try_i128i128, i128, i128 }
```

## Block 101
**Metadata**: AST_ID=101 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_always_ok! { test_try_usizeusize, usize, usize }
```

## Block 102
**Metadata**: AST_ID=102 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_always_ok! { test_try_isizeisize, isize, isize }
```

## Block 103
**Metadata**: AST_ID=103 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=37 | LINES=45

```rust
assume_usize_width! {
    test_impl_try_from_always_ok! { test_try_u8usize, u8, usize }
    test_impl_try_from_always_ok! { test_try_u8isize, u8, isize }
    test_impl_try_from_always_ok! { test_try_i8isize, i8, isize }

    test_impl_try_from_always_ok! { test_try_u16usize, u16, usize }
    test_impl_try_from_always_ok! { test_try_i16isize, i16, isize }

    test_impl_try_from_always_ok! { test_try_usizeu64, usize, u64 }
    test_impl_try_from_always_ok! { test_try_usizeu128, usize, u128 }
    test_impl_try_from_always_ok! { test_try_usizei128, usize, i128 }

    test_impl_try_from_always_ok! { test_try_isizei64, isize, i64 }
    test_impl_try_from_always_ok! { test_try_isizei128, isize, i128 }

    cfg_block!(
        #[cfg(target_pointer_width = "16")] {
            test_impl_try_from_always_ok! { test_try_usizeu16, usize, u16 }
            test_impl_try_from_always_ok! { test_try_isizei16, isize, i16 }
            test_impl_try_from_always_ok! { test_try_usizeu32, usize, u32 }
            test_impl_try_from_always_ok! { test_try_usizei32, usize, i32 }
            test_impl_try_from_always_ok! { test_try_isizei32, isize, i32 }
            test_impl_try_from_always_ok! { test_try_usizei64, usize, i64 }
        }

        #[cfg(target_pointer_width = "32")] {
            test_impl_try_from_always_ok! { test_try_u16isize, u16, isize }
            test_impl_try_from_always_ok! { test_try_usizeu32, usize, u32 }
            test_impl_try_from_always_ok! { test_try_isizei32, isize, i32 }
            test_impl_try_from_always_ok! { test_try_u32usize, u32, usize }
            test_impl_try_from_always_ok! { test_try_i32isize, i32, isize }
            test_impl_try_from_always_ok! { test_try_usizei64, usize, i64 }
        }

        #[cfg(target_pointer_width = "64")] {
            test_impl_try_from_always_ok! { test_try_u16isize, u16, isize }
            test_impl_try_from_always_ok! { test_try_u32usize, u32, usize }
            test_impl_try_from_always_ok! { test_try_u32isize, u32, isize }
            test_impl_try_from_always_ok! { test_try_i32isize, i32, isize }
            test_impl_try_from_always_ok! { test_try_u64usize, u64, usize }
            test_impl_try_from_always_ok! { test_try_i64isize, i64, isize }
        }
    );
}
```

## Block 104
**Metadata**: AST_ID=104 | TYPE=FUNCTION | NAME=$fn_name | COMPLEXITY=10 | LINES=17

```rust
/// Conversions where max of $source can be represented as $target,
macro_rules! test_impl_try_from_signed_to_unsigned_upper_ok {
    ($fn_name:ident, $source:ty, $target:ty) => {
        #[test]
        fn $fn_name() {
            let max = <$source>::MAX;
            let min = <$source>::MIN;
            let zero: $source = 0;
            let neg_one: $source = -1;
            assert_eq!(<$target as TryFrom<$source>>::try_from(max).unwrap(), max as $target);
            assert!(<$target as TryFrom<$source>>::try_from(min).is_err());
            assert_eq!(<$target as TryFrom<$source>>::try_from(zero).unwrap(), zero as $target);
            assert!(<$target as TryFrom<$source>>::try_from(neg_one).is_err());
        }
    };
}
```

## Block 105
**Metadata**: AST_ID=105 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8u8, i8, u8 }
```

## Block 106
**Metadata**: AST_ID=106 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8u16, i8, u16 }
```

## Block 107
**Metadata**: AST_ID=107 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8u32, i8, u32 }
```

## Block 108
**Metadata**: AST_ID=108 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8u64, i8, u64 }
```

## Block 109
**Metadata**: AST_ID=109 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8u128, i8, u128 }
```

## Block 110
**Metadata**: AST_ID=110 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i16u16, i16, u16 }
```

## Block 111
**Metadata**: AST_ID=111 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i16u32, i16, u32 }
```

## Block 112
**Metadata**: AST_ID=112 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i16u64, i16, u64 }
```

## Block 113
**Metadata**: AST_ID=113 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i16u128, i16, u128 }
```

## Block 114
**Metadata**: AST_ID=114 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i32u32, i32, u32 }
```

## Block 115
**Metadata**: AST_ID=115 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i32u64, i32, u64 }
```

## Block 116
**Metadata**: AST_ID=116 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i32u128, i32, u128 }
```

## Block 117
**Metadata**: AST_ID=117 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i64u64, i64, u64 }
```

## Block 118
**Metadata**: AST_ID=118 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i64u128, i64, u128 }
```

## Block 119
**Metadata**: AST_ID=119 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i128u128, i128, u128 }
```

## Block 120
**Metadata**: AST_ID=120 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=18 | LINES=27

```rust
assume_usize_width! {
    test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i8usize, i8, usize }
    test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i16usize, i16, usize }

    test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeu64, isize, u64 }
    test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeu128, isize, u128 }
    test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeusize, isize, usize }

    cfg_block!(
        #[cfg(target_pointer_width = "16")] {
            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeu16, isize, u16 }
            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeu32, isize, u32 }
        }

        #[cfg(target_pointer_width = "32")] {
            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeu32, isize, u32 }

            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i32usize, i32, usize }
        }

        #[cfg(target_pointer_width = "64")] {
            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i32usize, i32, usize }
            test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_i64usize, i64, usize }
        }
    );
}
```

## Block 121
**Metadata**: AST_ID=121 | TYPE=FUNCTION | NAME=$fn_name | COMPLEXITY=10 | LINES=16

```rust
/// Conversions where max of $source can not be represented as $target,
/// but min can.
macro_rules! test_impl_try_from_unsigned_to_signed_upper_err {
    ($fn_name:ident, $source:ty, $target:ty) => {
        #[test]
        fn $fn_name() {
            let max = <$source>::MAX;
            let min = <$source>::MIN;
            let zero: $source = 0;
            assert!(<$target as TryFrom<$source>>::try_from(max).is_err());
            assert_eq!(<$target as TryFrom<$source>>::try_from(min).unwrap(), min as $target);
            assert_eq!(<$target as TryFrom<$source>>::try_from(zero).unwrap(), zero as $target);
        }
    };
}
```

## Block 122
**Metadata**: AST_ID=122 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u8i8, u8, i8 }
```

## Block 123
**Metadata**: AST_ID=123 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u16i8, u16, i8 }
```

## Block 124
**Metadata**: AST_ID=124 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u16i16, u16, i16 }
```

## Block 125
**Metadata**: AST_ID=125 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u32i8, u32, i8 }
```

## Block 126
**Metadata**: AST_ID=126 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u32i16, u32, i16 }
```

## Block 127
**Metadata**: AST_ID=127 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u32i32, u32, i32 }
```

## Block 128
**Metadata**: AST_ID=128 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u64i8, u64, i8 }
```

## Block 129
**Metadata**: AST_ID=129 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u64i16, u64, i16 }
```

## Block 130
**Metadata**: AST_ID=130 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u64i32, u64, i32 }
```

## Block 131
**Metadata**: AST_ID=131 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u64i64, u64, i64 }
```

## Block 132
**Metadata**: AST_ID=132 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128i8, u128, i8 }
```

## Block 133
**Metadata**: AST_ID=133 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128i16, u128, i16 }
```

## Block 134
**Metadata**: AST_ID=134 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128i32, u128, i32 }
```

## Block 135
**Metadata**: AST_ID=135 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128i64, u128, i64 }
```

## Block 136
**Metadata**: AST_ID=136 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128i128, u128, i128 }
```

## Block 137
**Metadata**: AST_ID=137 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=18 | LINES=26

```rust
assume_usize_width! {
    test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u64isize, u64, isize }
    test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u128isize, u128, isize }

    test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizei8, usize, i8 }
    test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizei16, usize, i16 }
    test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizeisize, usize, isize }

    cfg_block!(
        #[cfg(target_pointer_width = "16")] {
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u16isize, u16, isize }
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u32isize, u32, isize }
        }

        #[cfg(target_pointer_width = "32")] {
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_u32isize, u32, isize }
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizei32, usize, i32 }
        }

        #[cfg(target_pointer_width = "64")] {
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizei32, usize, i32 }
            test_impl_try_from_unsigned_to_signed_upper_err! { test_try_usizei64, usize, i64 }
        }
    );
}
```

## Block 138
**Metadata**: AST_ID=138 | TYPE=FUNCTION | NAME=$fn_name | COMPLEXITY=14 | LINES=27

```rust
/// Conversions where min/max of $source can not be represented as $target.
macro_rules! test_impl_try_from_same_sign_err {
    ($fn_name:ident, $source:ty, $target:ty) => {
        #[test]
        fn $fn_name() {
            let max = <$source>::MAX;
            let min = <$source>::MIN;
            let zero: $source = 0;
            let t_max = <$target>::MAX;
            let t_min = <$target>::MIN;
            assert!(<$target as TryFrom<$source>>::try_from(max).is_err());
            if min != 0 {
                assert!(<$target as TryFrom<$source>>::try_from(min).is_err());
            }
            assert_eq!(<$target as TryFrom<$source>>::try_from(zero).unwrap(), zero as $target);
            assert_eq!(
                <$target as TryFrom<$source>>::try_from(t_max as $source).unwrap(),
                t_max as $target
            );
            assert_eq!(
                <$target as TryFrom<$source>>::try_from(t_min as $source).unwrap(),
                t_min as $target
            );
        }
    };
}
```

## Block 139
**Metadata**: AST_ID=139 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_same_sign_err! { test_try_u16u8, u16, u8 }
```

## Block 140
**Metadata**: AST_ID=140 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_same_sign_err! { test_try_u32u8, u32, u8 }
```

## Block 141
**Metadata**: AST_ID=141 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_u32u16, u32, u16 }
```

## Block 142
**Metadata**: AST_ID=142 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_same_sign_err! { test_try_u64u8, u64, u8 }
```

## Block 143
**Metadata**: AST_ID=143 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_u64u16, u64, u16 }
```

## Block 144
**Metadata**: AST_ID=144 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_u64u32, u64, u32 }
```

## Block 145
**Metadata**: AST_ID=145 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_same_sign_err! { test_try_u128u8, u128, u8 }
```

## Block 146
**Metadata**: AST_ID=146 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_u128u16, u128, u16 }
```

## Block 147
**Metadata**: AST_ID=147 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_u128u32, u128, u32 }
```

## Block 148
**Metadata**: AST_ID=148 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_u128u64, u128, u64 }
```

## Block 149
**Metadata**: AST_ID=149 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_same_sign_err! { test_try_i16i8, i16, i8 }
```

## Block 150
**Metadata**: AST_ID=150 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_isizei8, isize, i8 }
```

## Block 151
**Metadata**: AST_ID=151 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_same_sign_err! { test_try_i32i8, i32, i8 }
```

## Block 152
**Metadata**: AST_ID=152 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_i32i16, i32, i16 }
```

## Block 153
**Metadata**: AST_ID=153 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_same_sign_err! { test_try_i64i8, i64, i8 }
```

## Block 154
**Metadata**: AST_ID=154 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_i64i16, i64, i16 }
```

## Block 155
**Metadata**: AST_ID=155 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_i64i32, i64, i32 }
```

## Block 156
**Metadata**: AST_ID=156 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_same_sign_err! { test_try_i128i8, i128, i8 }
```

## Block 157
**Metadata**: AST_ID=157 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_i128i16, i128, i16 }
```

## Block 158
**Metadata**: AST_ID=158 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_i128i32, i128, i32 }
```

## Block 159
**Metadata**: AST_ID=159 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_same_sign_err! { test_try_i128i64, i128, i64 }
```

## Block 160
**Metadata**: AST_ID=160 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=22 | LINES=32

```rust
assume_usize_width! {
    test_impl_try_from_same_sign_err! { test_try_usizeu8, usize, u8 }
    test_impl_try_from_same_sign_err! { test_try_u128usize, u128, usize }
    test_impl_try_from_same_sign_err! { test_try_i128isize, i128, isize }

    cfg_block!(
        #[cfg(target_pointer_width = "16")] {
            test_impl_try_from_same_sign_err! { test_try_u32usize, u32, usize }
            test_impl_try_from_same_sign_err! { test_try_u64usize, u64, usize }

            test_impl_try_from_same_sign_err! { test_try_i32isize, i32, isize }
            test_impl_try_from_same_sign_err! { test_try_i64isize, i64, isize }
        }

        #[cfg(target_pointer_width = "32")] {
            test_impl_try_from_same_sign_err! { test_try_u64usize, u64, usize }
            test_impl_try_from_same_sign_err! { test_try_usizeu16, usize, u16 }

            test_impl_try_from_same_sign_err! { test_try_i64isize, i64, isize }
            test_impl_try_from_same_sign_err! { test_try_isizei16, isize, i16 }
        }

        #[cfg(target_pointer_width = "64")] {
            test_impl_try_from_same_sign_err! { test_try_usizeu16, usize, u16 }
            test_impl_try_from_same_sign_err! { test_try_usizeu32, usize, u32 }

            test_impl_try_from_same_sign_err! { test_try_isizei16, isize, i16 }
            test_impl_try_from_same_sign_err! { test_try_isizei32, isize, i32 }
        }
    );
}
```

## Block 161
**Metadata**: AST_ID=161 | TYPE=FUNCTION | NAME=$fn_name | COMPLEXITY=11 | LINES=26

```rust
/// Conversions where neither the min nor the max of $source can be represented by
/// $target, but max/min of the target can be represented by the source.
macro_rules! test_impl_try_from_signed_to_unsigned_err {
    ($fn_name:ident, $source:ty, $target:ty) => {
        #[test]
        fn $fn_name() {
            let max = <$source>::MAX;
            let min = <$source>::MIN;
            let zero: $source = 0;
            let t_max = <$target>::MAX;
            let t_min = <$target>::MIN;
            assert!(<$target as TryFrom<$source>>::try_from(max).is_err());
            assert!(<$target as TryFrom<$source>>::try_from(min).is_err());
            assert_eq!(<$target as TryFrom<$source>>::try_from(zero).unwrap(), zero as $target);
            assert_eq!(
                <$target as TryFrom<$source>>::try_from(t_max as $source).unwrap(),
                t_max as $target
            );
            assert_eq!(
                <$target as TryFrom<$source>>::try_from(t_min as $source).unwrap(),
                t_min as $target
            );
        }
    };
}
```

## Block 162
**Metadata**: AST_ID=162 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_signed_to_unsigned_err! { test_try_i16u8, i16, u8 }
```

## Block 163
**Metadata**: AST_ID=163 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_signed_to_unsigned_err! { test_try_i32u8, i32, u8 }
```

## Block 164
**Metadata**: AST_ID=164 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_err! { test_try_i32u16, i32, u16 }
```

## Block 165
**Metadata**: AST_ID=165 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_signed_to_unsigned_err! { test_try_i64u8, i64, u8 }
```

## Block 166
**Metadata**: AST_ID=166 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_err! { test_try_i64u16, i64, u16 }
```

## Block 167
**Metadata**: AST_ID=167 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_err! { test_try_i64u32, i64, u32 }
```

## Block 168
**Metadata**: AST_ID=168 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
test_impl_try_from_signed_to_unsigned_err! { test_try_i128u8, i128, u8 }
```

## Block 169
**Metadata**: AST_ID=169 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_err! { test_try_i128u16, i128, u16 }
```

## Block 170
**Metadata**: AST_ID=170 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_err! { test_try_i128u32, i128, u32 }
```

## Block 171
**Metadata**: AST_ID=171 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
test_impl_try_from_signed_to_unsigned_err! { test_try_i128u64, i128, u64 }
```

## Block 172
**Metadata**: AST_ID=172 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=15 | LINES=21

```rust
assume_usize_width! {
    test_impl_try_from_signed_to_unsigned_err! { test_try_isizeu8, isize, u8 }
    test_impl_try_from_signed_to_unsigned_err! { test_try_i128usize, i128, usize }

    cfg_block! {
        #[cfg(target_pointer_width = "16")] {
            test_impl_try_from_signed_to_unsigned_err! { test_try_i32usize, i32, usize }
            test_impl_try_from_signed_to_unsigned_err! { test_try_i64usize, i64, usize }
        }
        #[cfg(target_pointer_width = "32")] {
            test_impl_try_from_signed_to_unsigned_err! { test_try_i64usize, i64, usize }

            test_impl_try_from_signed_to_unsigned_err! { test_try_isizeu16, isize, u16 }
        }
        #[cfg(target_pointer_width = "64")] {
            test_impl_try_from_signed_to_unsigned_err! { test_try_isizeu16, isize, u16 }
            test_impl_try_from_signed_to_unsigned_err! { test_try_isizeu32, isize, u32 }
        }
    }
}
```

---
*Generated by AST tracing system*
