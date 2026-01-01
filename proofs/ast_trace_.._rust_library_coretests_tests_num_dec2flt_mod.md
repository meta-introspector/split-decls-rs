# AST Trace: ../rust/library/coretests/tests/num/dec2flt/mod.rs

Generated 15 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=MODULE | NAME=UNNAMED | COMPLEXITY=25 | LINES=33

```rust
#![allow(overflowing_literals)]

mod decimal;
mod decimal_seq;
mod float;
mod lemire;
mod parse;

// Take a float literal, turn it into a string in various ways (that are all trusted
// to be correct) and see if those strings are parsed back to the value of the literal.
// Requires a *polymorphic literal*, i.e., one that can serve as f64 as well as f32.
macro_rules! test_literal {
    ($x: expr) => {{
        #[cfg(target_has_reliable_f16)]
        let x16: f16 = $x;
        let x32: f32 = $x;
        let x64: f64 = $x;
        let inputs = &[stringify!($x).into(), format!("{:?}", x64), format!("{:e}", x64)];

        for input in inputs {
            assert_eq!(input.parse(), Ok(x64), "failed f64 {input}");
            assert_eq!(input.parse(), Ok(x32), "failed f32 {input}");
            #[cfg(target_has_reliable_f16)]
            assert_eq!(input.parse(), Ok(x16), "failed f16 {input}");

            let neg_input = format!("-{input}");
            assert_eq!(neg_input.parse(), Ok(-x64), "failed f64 {neg_input}");
            assert_eq!(neg_input.parse(), Ok(-x32), "failed f32 {neg_input}");
            #[cfg(target_has_reliable_f16)]
            assert_eq!(neg_input.parse(), Ok(-x16), "failed f16 {neg_input}");
        }
    }};
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=ordinary | COMPLEXITY=2 | LINES=10

```rust
#[test]
fn ordinary() {
    test_literal!(1.0);
    test_literal!(3e-5);
    test_literal!(0.1);
    test_literal!(12345.);
    test_literal!(0.9999999);
    test_literal!(2.2250738585072014e-308);
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=special_code_paths | COMPLEXITY=4 | LINES=9

```rust
#[test]
fn special_code_paths() {
    test_literal!(36893488147419103229.0); // 2^65 - 3, triggers half-to-even with even significand
    test_literal!(101e-33); // Triggers the tricky underflow case in AlgorithmM (for f32)
    test_literal!(1e23); // Triggers AlgorithmR
    test_literal!(2075e23); // Triggers another path through AlgorithmR
    test_literal!(8713e-23); // ... and yet another.
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=large | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn large() {
    test_literal!(1e300);
    test_literal!(123456789.34567e250);
    test_literal!(943794359898089732078308743689303290943794359843568973207830874368930329.);
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=subnormals | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn subnormals() {
    test_literal!(5e-324);
    test_literal!(91e-324);
    test_literal!(1e-322);
    test_literal!(13245643e-320);
    test_literal!(2.22507385851e-308);
    test_literal!(2.1e-308);
    test_literal!(4.9406564584124654e-324);
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=infinity | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn infinity() {
    test_literal!(1e400);
    test_literal!(1e309);
    test_literal!(2e308);
    test_literal!(1.7976931348624e308);
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=zero | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn zero() {
    test_literal!(0.0);
    test_literal!(1e-325);
    test_literal!(1e-326);
    test_literal!(1e-500);
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=fast_path_correct | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn fast_path_correct() {
    // This number triggers the fast path and is handled incorrectly when compiling on
    // x86 without SSE2 (i.e., using the x87 FPU stack).
    test_literal!(1.448997445238699);
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=lonely_dot | COMPLEXITY=2 | LINES=10

```rust
// FIXME(f16_f128): remove gates once tests work on all targets

#[test]
fn lonely_dot() {
    #[cfg(target_has_reliable_f16)]
    assert!(".".parse::<f16>().is_err());
    assert!(".".parse::<f32>().is_err());
    assert!(".".parse::<f64>().is_err());
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=exponentiated_dot | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn exponentiated_dot() {
    #[cfg(target_has_reliable_f16)]
    assert!(".e0".parse::<f16>().is_err());
    assert!(".e0".parse::<f32>().is_err());
    assert!(".e0".parse::<f64>().is_err());
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=lonely_sign | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn lonely_sign() {
    #[cfg(target_has_reliable_f16)]
    assert!("+".parse::<f16>().is_err());
    assert!("-".parse::<f32>().is_err());
    assert!("+".parse::<f64>().is_err());
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=whitespace | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn whitespace() {
    #[cfg(target_has_reliable_f16)]
    assert!("1.0 ".parse::<f16>().is_err());
    assert!(" 1.0".parse::<f32>().is_err());
    assert!("1.0 ".parse::<f64>().is_err());
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=nan | COMPLEXITY=3 | LINES=15

```rust
#[test]
fn nan() {
    #[cfg(target_has_reliable_f16)]
    {
        assert!("NaN".parse::<f16>().unwrap().is_nan());
        assert!("-NaN".parse::<f16>().unwrap().is_nan());
    }

    assert!("NaN".parse::<f32>().unwrap().is_nan());
    assert!("-NaN".parse::<f32>().unwrap().is_nan());

    assert!("NaN".parse::<f64>().unwrap().is_nan());
    assert!("-NaN".parse::<f64>().unwrap().is_nan());
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=inf | COMPLEXITY=3 | LINES=15

```rust
#[test]
fn inf() {
    #[cfg(target_has_reliable_f16)]
    {
        assert_eq!("inf".parse(), Ok(f16::INFINITY));
        assert_eq!("-inf".parse(), Ok(f16::NEG_INFINITY));
    }

    assert_eq!("inf".parse(), Ok(f32::INFINITY));
    assert_eq!("-inf".parse(), Ok(f32::NEG_INFINITY));

    assert_eq!("inf".parse(), Ok(f64::INFINITY));
    assert_eq!("-inf".parse(), Ok(f64::NEG_INFINITY));
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=massive_exponent | COMPLEXITY=13 | LINES=21

```rust
#[test]
fn massive_exponent() {
    #[cfg(target_has_reliable_f16)]
    {
        let max = i16::MAX;
        assert_eq!(format!("1e{max}000").parse(), Ok(f16::INFINITY));
        assert_eq!(format!("1e-{max}000").parse(), Ok(0.0f16));
        assert_eq!(format!("1e{max}000").parse(), Ok(f16::INFINITY));
    }

    let max = i32::MAX;
    assert_eq!(format!("1e{max}000").parse(), Ok(f32::INFINITY));
    assert_eq!(format!("1e-{max}000").parse(), Ok(0.0f32));
    assert_eq!(format!("1e{max}000").parse(), Ok(f32::INFINITY));

    let max = i64::MAX;
    assert_eq!(format!("1e{max}000").parse(), Ok(f64::INFINITY));
    assert_eq!(format!("1e-{max}000").parse(), Ok(0.0f64));
    assert_eq!(format!("1e{max}000").parse(), Ok(f64::INFINITY));
}
```

---
*Generated by AST tracing system*
