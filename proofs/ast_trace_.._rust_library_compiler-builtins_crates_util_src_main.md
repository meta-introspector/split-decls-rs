# AST Trace: ../rust/library/compiler-builtins/crates/util/src/main.rs

Generated 19 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=11

```rust
//! Helper CLI utility for common tasks.

#![cfg_attr(f16_enabled, feature(f16))]
#![cfg_attr(f128_enabled, feature(f128))]

use std::any::type_name;
use std::env;
use std::num::ParseIntError;
use std::str::FromStr;

use libm::support::{Hexf, hf32, hf64};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
#[cfg(feature = "build-mpfr")]
use libm_test::mpfloat::MpOp;
use libm_test::{MathOp, TupleCall};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
#[cfg(feature = "build-mpfr")]
use rug::az::{self, Az};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=main | COMPLEXITY=10 | LINES=26

```rust
const USAGE: &str = "\
usage:

cargo run -p util -- <SUBCOMMAND>

SUBCOMMAND:
    eval <BASIS> <OP> inputs...
        Evaulate the expression with a given basis. This can be useful for
        running routines with a debugger, or quickly checking input. Examples:
        * eval musl sinf 1.234 # print the results of musl sinf(1.234f32)
        * eval mpfr pow 1.234 2.432 # print the results of mpfr pow(1.234, 2.432)
";

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let str_args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    match &str_args.as_slice()[1..] {
        ["eval", basis, op, inputs @ ..] => do_eval(basis, op, inputs),
        _ => {
            println!("{USAGE}\nunrecognized input `{str_args:?}`");
            std::process::exit(1);
        }
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=25 | LINES=38

```rust
macro_rules! handle_call {
    (
        fn_name: $fn_name:ident,
        CFn: $CFn:ty,
        RustFn: $RustFn:ty,
        RustArgs: $RustArgs:ty,
        attrs: [$($attr:meta),*],
        extra: ($basis:ident, $op:ident, $inputs:ident),
        fn_extra: $musl_fn:expr,
    ) => {
        $(#[$attr])*
        if $op == stringify!($fn_name) {
            type Op = libm_test::op::$fn_name::Routine;

            let input = <$RustArgs>::parse($inputs);
            let libm_fn: <Op as MathOp>::RustFn = libm::$fn_name;

            let output = match $basis {
                "libm" => input.call_intercept_panics(libm_fn),
                #[cfg(feature = "build-musl")]
                "musl" => {
                    let musl_fn: <Op as MathOp>::CFn =
                        $musl_fn.unwrap_or_else(|| panic!("no musl function for {}", $op));
                    input.call(musl_fn)
                }
                #[cfg(feature = "build-mpfr")]
                "mpfr" => {
                    let mut mp = <Op as MpOp>::new_mp();
                    Op::run(&mut mp, input)
                }
                _ => panic!("unrecognized or disabled basis '{}'", $basis),
            };
            println!("{output:?} {:x}", Hexf(output));
            return;
        }
    };
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=do_eval | COMPLEXITY=9 | LINES=27

```rust
/// Evaluate the specified operation with a given basis.
fn do_eval(basis: &str, op: &str, inputs: &[&str]) {
    libm_macros::for_each_function! {
        callback: handle_call,
        emit_types: [CFn, RustFn, RustArgs],
        extra: (basis, op, inputs),
        fn_extra: match MACRO_FN_NAME {
            // Not provided by musl
            fmaximum
            | fmaximum_num
            | fmaximum_numf
            | fmaximumf
            | fminimum
            | fminimum_num
            | fminimum_numf
            | fminimumf
            | roundeven
            | roundevenf
            | ALL_F16
            | ALL_F128 => None,
            _ => Some(musl_math_sys::MACRO_FN_NAME)
        }
    }

    panic!("no operation matching {op}");
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=parse | COMPLEXITY=2 | LINES=5

```rust
/// Parse a tuple from a space-delimited string.
trait ParseTuple {
    fn parse(input: &[&str]) -> Self;
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=parse | COMPLEXITY=35 | LINES=39

```rust
macro_rules! impl_parse_tuple {
    ($ty:ty) => {
        impl ParseTuple for ($ty,) {
            fn parse(input: &[&str]) -> Self {
                assert_eq!(input.len(), 1, "expected a single argument, got {input:?}");
                (parse(input, 0),)
            }
        }

        impl ParseTuple for ($ty, $ty) {
            fn parse(input: &[&str]) -> Self {
                assert_eq!(input.len(), 2, "expected two arguments, got {input:?}");
                (parse(input, 0), parse(input, 1))
            }
        }

        impl ParseTuple for ($ty, i32) {
            fn parse(input: &[&str]) -> Self {
                assert_eq!(input.len(), 2, "expected two arguments, got {input:?}");
                (parse(input, 0), parse(input, 1))
            }
        }

        impl ParseTuple for (i32, $ty) {
            fn parse(input: &[&str]) -> Self {
                assert_eq!(input.len(), 2, "expected two arguments, got {input:?}");
                (parse(input, 0), parse(input, 1))
            }
        }

        impl ParseTuple for ($ty, $ty, $ty) {
            fn parse(input: &[&str]) -> Self {
                assert_eq!(input.len(), 3, "expected three arguments, got {input:?}");
                (parse(input, 0), parse(input, 1), parse(input, 2))
            }
        }
    };
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=parse | COMPLEXITY=35 | LINES=45

```rust
#[allow(unused_macros)]
#[cfg(feature = "build-mpfr")]
macro_rules! impl_parse_tuple_via_rug {
    ($ty:ty) => {
        impl ParseTuple for ($ty,) {
            fn parse(input: &[&str]) -> Self {
                assert_eq!(input.len(), 1, "expected a single argument, got {input:?}");
                (parse_rug(input, 0),)
            }
        }

        impl ParseTuple for ($ty, $ty) {
            fn parse(input: &[&str]) -> Self {
                assert_eq!(input.len(), 2, "expected two arguments, got {input:?}");
                (parse_rug(input, 0), parse_rug(input, 1))
            }
        }

        impl ParseTuple for ($ty, i32) {
            fn parse(input: &[&str]) -> Self {
                assert_eq!(input.len(), 2, "expected two arguments, got {input:?}");
                (parse_rug(input, 0), parse(input, 1))
            }
        }

        impl ParseTuple for (i32, $ty) {
            fn parse(input: &[&str]) -> Self {
                assert_eq!(input.len(), 2, "expected two arguments, got {input:?}");
                (parse(input, 0), parse_rug(input, 1))
            }
        }

        impl ParseTuple for ($ty, $ty, $ty) {
            fn parse(input: &[&str]) -> Self {
                assert_eq!(input.len(), 3, "expected three arguments, got {input:?}");
                (
                    parse_rug(input, 0),
                    parse_rug(input, 1),
                    parse_rug(input, 2),
                )
            }
        }
    };
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=parse | COMPLEXITY=32 | LINES=37

```rust
// Fallback for when Rug is not built.
#[allow(unused_macros)]
#[cfg(not(feature = "build-mpfr"))]
macro_rules! impl_parse_tuple_via_rug {
    ($ty:ty) => {
        impl ParseTuple for ($ty,) {
            fn parse(_input: &[&str]) -> Self {
                panic!("parsing this type requires the `build-mpfr` feature")
            }
        }

        impl ParseTuple for ($ty, $ty) {
            fn parse(_input: &[&str]) -> Self {
                panic!("parsing this type requires the `build-mpfr` feature")
            }
        }

        impl ParseTuple for ($ty, i32) {
            fn parse(_input: &[&str]) -> Self {
                panic!("parsing this type requires the `build-mpfr` feature")
            }
        }

        impl ParseTuple for (i32, $ty) {
            fn parse(_input: &[&str]) -> Self {
                panic!("parsing this type requires the `build-mpfr` feature")
            }
        }

        impl ParseTuple for ($ty, $ty, $ty) {
            fn parse(_input: &[&str]) -> Self {
                panic!("parsing this type requires the `build-mpfr` feature")
            }
        }
    };
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=parse | COMPLEXITY=14 | LINES=25

```rust
impl_parse_tuple!(f32);
impl_parse_tuple!(f64);

#[cfg(f16_enabled)]
impl_parse_tuple_via_rug!(f16);
#[cfg(f128_enabled)]
impl_parse_tuple_via_rug!(f128);

/// Try to parse the number, printing a nice message on failure.
fn parse<T: FromStr + FromStrRadix>(input: &[&str], idx: usize) -> T {
    let s = input[idx];

    let msg = || format!("invalid {} input '{s}'", type_name::<T>());

    if s.starts_with("0x") || s.starts_with("-0x") {
        return T::from_str_radix(s, 16).unwrap_or_else(|_| panic!("{}", msg()));
    }

    if s.starts_with("0b") {
        return T::from_str_radix(s, 2).unwrap_or_else(|_| panic!("{}", msg()));
    }

    s.parse().unwrap_or_else(|_| panic!("{}", msg()))
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=parse_rug | COMPLEXITY=16 | LINES=25

```rust
/// Try to parse the float type going via `rug`, for `f16` and `f128` which don't yet implement
/// `FromStr`.
#[cfg(feature = "build-mpfr")]
fn parse_rug<F>(input: &[&str], idx: usize) -> F
where
    F: libm_test::Float + FromStrRadix,
    rug::Float: az::Cast<F>,
{
    let s = input[idx];

    let msg = || format!("invalid {} input '{s}'", type_name::<F>());

    if s.starts_with("0x") {
        return F::from_str_radix(s, 16).unwrap_or_else(|_| panic!("{}", msg()));
    }

    if s.starts_with("0b") {
        return F::from_str_radix(s, 2).unwrap_or_else(|_| panic!("{}", msg()));
    }

    let x = rug::Float::parse(s).unwrap_or_else(|_| panic!("{}", msg()));
    let x = rug::Float::with_val(F::BITS, x);
    x.az()
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=from_str_radix | COMPLEXITY=2 | LINES=4

```rust
trait FromStrRadix: Sized {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError>;
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=from_str_radix | COMPLEXITY=5 | LINES=7

```rust
impl FromStrRadix for i32 {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        let s = strip_radix_prefix(s, radix);
        i32::from_str_radix(s, radix)
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=from_str_radix | COMPLEXITY=8 | LINES=12

```rust
#[cfg(f16_enabled)]
impl FromStrRadix for f16 {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        if radix == 16 && s.contains("p") {
            return Ok(libm::support::hf16(s));
        }

        let s = strip_radix_prefix(s, radix);
        u16::from_str_radix(s, radix).map(Self::from_bits)
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=from_str_radix | COMPLEXITY=8 | LINES=12

```rust
impl FromStrRadix for f32 {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        if radix == 16 && s.contains("p") {
            // Parse as hex float
            return Ok(hf32(s));
        }

        let s = strip_radix_prefix(s, radix);
        u32::from_str_radix(s, radix).map(Self::from_bits)
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=from_str_radix | COMPLEXITY=8 | LINES=11

```rust
impl FromStrRadix for f64 {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        if s.contains("p") {
            return Ok(hf64(s));
        }

        let s = strip_radix_prefix(s, radix);
        u64::from_str_radix(s, radix).map(Self::from_bits)
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=from_str_radix | COMPLEXITY=8 | LINES=11

```rust
#[cfg(f128_enabled)]
impl FromStrRadix for f128 {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        if radix == 16 && s.contains("p") {
            return Ok(libm::support::hf128(s));
        }
        let s = strip_radix_prefix(s, radix);
        u128::from_str_radix(s, radix).map(Self::from_bits)
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=strip_radix_prefix | COMPLEXITY=9 | LINES=10

```rust
fn strip_radix_prefix(s: &str, radix: u32) -> &str {
    if radix == 16 {
        s.strip_prefix("0x").unwrap()
    } else if radix == 2 {
        s.strip_prefix("0b").unwrap()
    } else {
        s
    }
}
```

---
*Generated by AST tracing system*
