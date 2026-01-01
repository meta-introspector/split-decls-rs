# AST Trace: ../rust/library/stdarch/crates/core_arch/src/arm_shared/test_support.rs

Generated 16 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[cfg(target_arch = "arm")]
use crate::core_arch::arm::*;

#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
use crate::core_arch::aarch64::*;

use crate::core_arch::simd::*;
use std::{mem::transmute, vec::Vec};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
macro_rules! V_u8 {
    () => {
        vec![0x00u8, 0x01u8, 0x02u8, 0x0Fu8, 0x80u8, 0xF0u8, 0xFFu8]
    };
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=7

```rust
macro_rules! V_u16 {
    () => {
        vec![
            0x0000u16, 0x0101u16, 0x0202u16, 0x0F0Fu16, 0x8000u16, 0xF0F0u16, 0xFFFFu16,
        ]
    };
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=13

```rust
macro_rules! V_u32 {
    () => {
        vec![
            0x00000000u32,
            0x01010101u32,
            0x02020202u32,
            0x0F0F0F0Fu32,
            0x80000000u32,
            0xF0F0F0F0u32,
            0xFFFFFFFFu32,
        ]
    };
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=13

```rust
macro_rules! V_u64 {
    () => {
        vec![
            0x0000000000000000u64,
            0x0101010101010101u64,
            0x0202020202020202u64,
            0x0F0F0F0F0F0F0F0Fu64,
            0x8080808080808080u64,
            0xF0F0F0F0F0F0F0F0u64,
            0xFFFFFFFFFFFFFFFFu64,
        ]
    };
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=10

```rust
macro_rules! V_i8 {
    () => {
        vec![
            0x00i8, 0x01i8, 0x02i8, 0x0Fi8, -128i8, /* 0x80 */
            -16i8,  /* 0xF0 */
            -1i8,   /* 0xFF */
        ]
    };
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=9

```rust
macro_rules! V_i16 {
    () => {
        vec![
            0x0000i16, 0x0101i16, 0x0202i16, 0x0F0Fi16, -32768i16, /* 0x8000 */
            -3856i16,  /* 0xF0F0 */
            -1i16,     /* 0xFFF */
        ]
    };
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=13

```rust
macro_rules! V_i32 {
    () => {
        vec![
            0x00000000i32,
            0x01010101i32,
            0x02020202i32,
            0x0F0F0F0Fi32,
            -2139062144i32, /* 0x80000000 */
            -252645136i32,  /* 0xF0F0F0F0 */
            -1i32,          /* 0xFFFFFFFF */
        ]
    };
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=14

```rust
macro_rules! V_i64 {
    () => {
        vec![
            0x0000000000000000i64,
            0x0101010101010101i64,
            0x0202020202020202i64,
            0x0F0F0F0F0F0F0F0Fi64,
            -9223372036854775808i64, /* 0x8000000000000000 */
            -1152921504606846976i64, /* 0xF000000000000000 */
            -1i64,                   /* 0xFFFFFFFFFFFFFFFF */
        ]
    };
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=17

```rust
macro_rules! V_f32 {
    () => {
        vec![
            0.0f32,
            1.0f32,
            -1.0f32,
            1.2f32,
            2.4f32,
            f32::MAX,
            f32::MIN,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
        ]
    };
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=9 | LINES=6

```rust
macro_rules! to64 {
    ($t : ident) => {
        |v: $t| -> u64 { transmute(v) }
    };
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=9 | LINES=6

```rust
macro_rules! to128 {
    ($t : ident) => {
        |v: $t| -> u128 { transmute(v) }
    };
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=34

```rust
pub(crate) fn test<T, U, V, W, X>(
    vals: Vec<T>,
    fill1: fn(T) -> V,
    fill2: fn(U) -> W,
    cast: fn(W) -> X,
    test_fun: fn(V, V) -> W,
    verify_fun: fn(T, T) -> U,
) where
    T: Copy + core::fmt::Debug + std::cmp::PartialEq,
    U: Copy + core::fmt::Debug + std::cmp::PartialEq,
    V: Copy + core::fmt::Debug,
    W: Copy + core::fmt::Debug,
    X: Copy + core::fmt::Debug + std::cmp::PartialEq,
{
    let pairs = vals.iter().zip(vals.iter());

    for (i, j) in pairs {
        let a: V = fill1(*i);
        let b: V = fill1(*j);

        let actual_pre: W = test_fun(a, b);
        let expected_pre: W = fill2(verify_fun(*i, *j));

        let actual: X = cast(actual_pre);
        let expected: X = cast(expected_pre);

        assert_eq!(
            actual, expected,
            "[{:?}:{:?}] :\nf({:?}, {:?}) = {:?}\ng({:?}, {:?}) = {:?}\n",
            *i, *j, &a, &b, actual_pre, &a, &b, expected_pre
        );
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=10

```rust
macro_rules! gen_test_fn {
    ($n: ident, $t: ident, $u: ident, $v: ident, $w: ident, $x: ident, $vals: expr, $fill1: expr, $fill2: expr, $cast: expr) => {
        pub(crate) fn $n(test_fun: fn($v, $v) -> $w, verify_fun: fn($t, $t) -> $u) {
            unsafe {
                test::<$t, $u, $v, $w, $x>($vals, $fill1, $fill2, $cast, test_fun, verify_fun)
            };
        }
    };
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=24 | LINES=14

```rust
macro_rules! gen_fill_fn {
    ($id: ident, $el_width: expr, $num_els: expr, $in_t : ident, $out_t: ident, $cmp_t: ident) => {
        pub(crate) fn $id(val: $in_t) -> $out_t {
            let initial: [$in_t; $num_els] = [val; $num_els];
            let result: $cmp_t = unsafe { transmute(initial) };
            let result_out: $out_t = unsafe { transmute(result) };

            // println!("FILL: {:016x} as {} x {}: {:016x}", val.reverse_bits(), $el_width, $num_els, (result as u64).reverse_bits());

            result_out
        }
    };
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=19 | LINES=656

```rust
gen_fill_fn!(fill_u8, 8, 8, u8, uint8x8_t, u64);
gen_fill_fn!(fill_s8, 8, 8, i8, int8x8_t, u64);
gen_fill_fn!(fillq_u8, 8, 16, u8, uint8x16_t, u128);
gen_fill_fn!(fillq_s8, 8, 16, i8, int8x16_t, u128);

gen_fill_fn!(fill_u16, 16, 4, u16, uint16x4_t, u64);
gen_fill_fn!(fill_s16, 16, 4, i16, int16x4_t, u64);
gen_fill_fn!(fillq_u16, 16, 8, u16, uint16x8_t, u128);
gen_fill_fn!(fillq_s16, 16, 8, i16, int16x8_t, u128);

gen_fill_fn!(fill_u32, 32, 2, u32, uint32x2_t, u64);
gen_fill_fn!(fill_s32, 32, 2, i32, int32x2_t, u64);
gen_fill_fn!(fillq_u32, 32, 4, u32, uint32x4_t, u128);
gen_fill_fn!(fillq_s32, 32, 4, i32, int32x4_t, u128);

gen_fill_fn!(fill_u64, 64, 1, u64, uint64x1_t, u64);
gen_fill_fn!(fill_s64, 64, 1, i64, int64x1_t, u64);
gen_fill_fn!(fillq_u64, 64, 2, u64, uint64x2_t, u128);
gen_fill_fn!(fillq_s64, 64, 2, i64, int64x2_t, u128);

gen_fill_fn!(fill_f32, 32, 2, f32, float32x2_t, u64);
gen_fill_fn!(fillq_f32, 32, 4, f32, float32x4_t, u128);

gen_test_fn!(
    test_ari_u8,
    u8,
    u8,
    uint8x8_t,
    uint8x8_t,
    u64,
    V_u8!(),
    fill_u8,
    fill_u8,
    to64!(uint8x8_t)
);
gen_test_fn!(
    test_bit_u8,
    u8,
    u8,
    uint8x8_t,
    uint8x8_t,
    u64,
    V_u8!(),
    fill_u8,
    fill_u8,
    to64!(uint8x8_t)
);
gen_test_fn!(
    test_cmp_u8,
    u8,
    u8,
    uint8x8_t,
    uint8x8_t,
    u64,
    V_u8!(),
    fill_u8,
    fill_u8,
    to64!(uint8x8_t)
);
gen_test_fn!(
    testq_ari_u8,
    u8,
    u8,
    uint8x16_t,
    uint8x16_t,
    u128,
    V_u8!(),
    fillq_u8,
    fillq_u8,
    to128!(uint8x16_t)
);
gen_test_fn!(
    testq_bit_u8,
    u8,
    u8,
    uint8x16_t,
    uint8x16_t,
    u128,
    V_u8!(),
    fillq_u8,
    fillq_u8,
    to128!(uint8x16_t)
);
gen_test_fn!(
    testq_cmp_u8,
    u8,
    u8,
    uint8x16_t,
    uint8x16_t,
    u128,
    V_u8!(),
    fillq_u8,
    fillq_u8,
    to128!(uint8x16_t)
);

gen_test_fn!(
    test_ari_s8,
    i8,
    i8,
    int8x8_t,
    int8x8_t,
    u64,
    V_i8!(),
    fill_s8,
    fill_s8,
    to64!(int8x8_t)
);
gen_test_fn!(
    test_bit_s8,
    i8,
    i8,
    int8x8_t,
    int8x8_t,
    u64,
    V_i8!(),
    fill_s8,
    fill_s8,
    to64!(int8x8_t)
);
gen_test_fn!(
    test_cmp_s8,
    i8,
    u8,
    int8x8_t,
    uint8x8_t,
    u64,
    V_i8!(),
    fill_s8,
    fill_u8,
    to64!(uint8x8_t)
);
gen_test_fn!(
    testq_ari_s8,
    i8,
    i8,
    int8x16_t,
    int8x16_t,
    u128,
    V_i8!(),
    fillq_s8,
    fillq_s8,
    to128!(int8x16_t)
);
gen_test_fn!(
    testq_bit_s8,
    i8,
    i8,
    int8x16_t,
    int8x16_t,
    u128,
    V_i8!(),
    fillq_s8,
    fillq_s8,
    to128!(int8x16_t)
);
gen_test_fn!(
    testq_cmp_s8,
    i8,
    u8,
    int8x16_t,
    uint8x16_t,
    u128,
    V_i8!(),
    fillq_s8,
    fillq_u8,
    to128!(uint8x16_t)
);

gen_test_fn!(
    test_ari_u16,
    u16,
    u16,
    uint16x4_t,
    uint16x4_t,
    u64,
    V_u16!(),
    fill_u16,
    fill_u16,
    to64!(uint16x4_t)
);
gen_test_fn!(
    test_bit_u16,
    u16,
    u16,
    uint16x4_t,
    uint16x4_t,
    u64,
    V_u16!(),
    fill_u16,
    fill_u16,
    to64!(uint16x4_t)
);
gen_test_fn!(
    test_cmp_u16,
    u16,
    u16,
    uint16x4_t,
    uint16x4_t,
    u64,
    V_u16!(),
    fill_u16,
    fill_u16,
    to64!(uint16x4_t)
);
gen_test_fn!(
    testq_ari_u16,
    u16,
    u16,
    uint16x8_t,
    uint16x8_t,
    u128,
    V_u16!(),
    fillq_u16,
    fillq_u16,
    to128!(uint16x8_t)
);
gen_test_fn!(
    testq_bit_u16,
    u16,
    u16,
    uint16x8_t,
    uint16x8_t,
    u128,
    V_u16!(),
    fillq_u16,
    fillq_u16,
    to128!(uint16x8_t)
);
gen_test_fn!(
    testq_cmp_u16,
    u16,
    u16,
    uint16x8_t,
    uint16x8_t,
    u128,
    V_u16!(),
    fillq_u16,
    fillq_u16,
    to128!(uint16x8_t)
);

gen_test_fn!(
    test_ari_s16,
    i16,
    i16,
    int16x4_t,
    int16x4_t,
    u64,
    V_i16!(),
    fill_s16,
    fill_s16,
    to64!(int16x4_t)
);
gen_test_fn!(
    test_bit_s16,
    i16,
    i16,
    int16x4_t,
    int16x4_t,
    u64,
    V_i16!(),
    fill_s16,
    fill_s16,
    to64!(int16x4_t)
);
gen_test_fn!(
    test_cmp_s16,
    i16,
    u16,
    int16x4_t,
    uint16x4_t,
    u64,
    V_i16!(),
    fill_s16,
    fill_u16,
    to64!(uint16x4_t)
);
gen_test_fn!(
    testq_ari_s16,
    i16,
    i16,
    int16x8_t,
    int16x8_t,
    u128,
    V_i16!(),
    fillq_s16,
    fillq_s16,
    to128!(int16x8_t)
);
gen_test_fn!(
    testq_bit_s16,
    i16,
    i16,
    int16x8_t,
    int16x8_t,
    u128,
    V_i16!(),
    fillq_s16,
    fillq_s16,
    to128!(int16x8_t)
);
gen_test_fn!(
    testq_cmp_s16,
    i16,
    u16,
    int16x8_t,
    uint16x8_t,
    u128,
    V_i16!(),
    fillq_s16,
    fillq_u16,
    to128!(uint16x8_t)
);

gen_test_fn!(
    test_ari_u32,
    u32,
    u32,
    uint32x2_t,
    uint32x2_t,
    u64,
    V_u32!(),
    fill_u32,
    fill_u32,
    to64!(uint32x2_t)
);
gen_test_fn!(
    test_bit_u32,
    u32,
    u32,
    uint32x2_t,
    uint32x2_t,
    u64,
    V_u32!(),
    fill_u32,
    fill_u32,
    to64!(uint32x2_t)
);
gen_test_fn!(
    test_cmp_u32,
    u32,
    u32,
    uint32x2_t,
    uint32x2_t,
    u64,
    V_u32!(),
    fill_u32,
    fill_u32,
    to64!(uint32x2_t)
);
gen_test_fn!(
    testq_ari_u32,
    u32,
    u32,
    uint32x4_t,
    uint32x4_t,
    u128,
    V_u32!(),
    fillq_u32,
    fillq_u32,
    to128!(uint32x4_t)
);
gen_test_fn!(
    testq_bit_u32,
    u32,
    u32,
    uint32x4_t,
    uint32x4_t,
    u128,
    V_u32!(),
    fillq_u32,
    fillq_u32,
    to128!(uint32x4_t)
);
gen_test_fn!(
    testq_cmp_u32,
    u32,
    u32,
    uint32x4_t,
    uint32x4_t,
    u128,
    V_u32!(),
    fillq_u32,
    fillq_u32,
    to128!(uint32x4_t)
);

gen_test_fn!(
    test_ari_s32,
    i32,
    i32,
    int32x2_t,
    int32x2_t,
    u64,
    V_i32!(),
    fill_s32,
    fill_s32,
    to64!(int32x2_t)
);
gen_test_fn!(
    test_bit_s32,
    i32,
    i32,
    int32x2_t,
    int32x2_t,
    u64,
    V_i32!(),
    fill_s32,
    fill_s32,
    to64!(int32x2_t)
);
gen_test_fn!(
    test_cmp_s32,
    i32,
    u32,
    int32x2_t,
    uint32x2_t,
    u64,
    V_i32!(),
    fill_s32,
    fill_u32,
    to64!(uint32x2_t)
);
gen_test_fn!(
    testq_ari_s32,
    i32,
    i32,
    int32x4_t,
    int32x4_t,
    u128,
    V_i32!(),
    fillq_s32,
    fillq_s32,
    to128!(int32x4_t)
);
gen_test_fn!(
    testq_bit_s32,
    i32,
    i32,
    int32x4_t,
    int32x4_t,
    u128,
    V_i32!(),
    fillq_s32,
    fillq_s32,
    to128!(int32x4_t)
);
gen_test_fn!(
    testq_cmp_s32,
    i32,
    u32,
    int32x4_t,
    uint32x4_t,
    u128,
    V_i32!(),
    fillq_s32,
    fillq_u32,
    to128!(uint32x4_t)
);

gen_test_fn!(
    test_ari_u64,
    u64,
    u64,
    uint64x1_t,
    uint64x1_t,
    u64,
    V_u64!(),
    fill_u64,
    fill_u64,
    to64!(uint64x1_t)
);
gen_test_fn!(
    test_bit_u64,
    u64,
    u64,
    uint64x1_t,
    uint64x1_t,
    u64,
    V_u64!(),
    fill_u64,
    fill_u64,
    to64!(uint64x1_t)
);
gen_test_fn!(
    test_cmp_u64,
    u64,
    u64,
    uint64x1_t,
    uint64x1_t,
    u64,
    V_u64!(),
    fill_u64,
    fill_u64,
    to64!(uint64x1_t)
);
gen_test_fn!(
    testq_ari_u64,
    u64,
    u64,
    uint64x2_t,
    uint64x2_t,
    u128,
    V_u64!(),
    fillq_u64,
    fillq_u64,
    to128!(uint64x2_t)
);
gen_test_fn!(
    testq_bit_u64,
    u64,
    u64,
    uint64x2_t,
    uint64x2_t,
    u128,
    V_u64!(),
    fillq_u64,
    fillq_u64,
    to128!(uint64x2_t)
);
gen_test_fn!(
    testq_cmp_u64,
    u64,
    u64,
    uint64x2_t,
    uint64x2_t,
    u128,
    V_u64!(),
    fillq_u64,
    fillq_u64,
    to128!(uint64x2_t)
);

gen_test_fn!(
    test_ari_s64,
    i64,
    i64,
    int64x1_t,
    int64x1_t,
    u64,
    V_i64!(),
    fill_s64,
    fill_s64,
    to64!(int64x1_t)
);
gen_test_fn!(
    test_bit_s64,
    i64,
    i64,
    int64x1_t,
    int64x1_t,
    u64,
    V_i64!(),
    fill_s64,
    fill_s64,
    to64!(int64x1_t)
);
gen_test_fn!(
    test_cmp_s64,
    i64,
    u64,
    int64x1_t,
    uint64x1_t,
    u64,
    V_i64!(),
    fill_s64,
    fill_u64,
    to64!(uint64x1_t)
);
gen_test_fn!(
    testq_ari_s64,
    i64,
    i64,
    int64x2_t,
    int64x2_t,
    u128,
    V_i64!(),
    fillq_s64,
    fillq_s64,
    to128!(int64x2_t)
);
gen_test_fn!(
    testq_bit_s64,
    i64,
    i64,
    int64x2_t,
    int64x2_t,
    u128,
    V_i64!(),
    fillq_s64,
    fillq_s64,
    to128!(int64x2_t)
);
gen_test_fn!(
    testq_cmp_s64,
    i64,
    u64,
    int64x2_t,
    uint64x2_t,
    u128,
    V_i64!(),
    fillq_s64,
    fillq_u64,
    to128!(uint64x2_t)
);

gen_test_fn!(
    test_ari_f32,
    f32,
    f32,
    float32x2_t,
    float32x2_t,
    u64,
    V_f32!(),
    fill_f32,
    fill_f32,
    to64!(float32x2_t)
);
gen_test_fn!(
    test_cmp_f32,
    f32,
    u32,
    float32x2_t,
    uint32x2_t,
    u64,
    V_f32!(),
    fill_f32,
    fill_u32,
    to64!(uint32x2_t)
);
gen_test_fn!(
    testq_ari_f32,
    f32,
    f32,
    float32x4_t,
    float32x4_t,
    u128,
    V_f32!(),
    fillq_f32,
    fillq_f32,
    to128!(float32x4_t)
);
gen_test_fn!(
    testq_cmp_f32,
    f32,
    u32,
    float32x4_t,
    uint32x4_t,
    u128,
    V_f32!(),
    fillq_f32,
    fillq_u32,
    to128!(uint32x4_t)
);
```

---
*Generated by AST tracing system*
