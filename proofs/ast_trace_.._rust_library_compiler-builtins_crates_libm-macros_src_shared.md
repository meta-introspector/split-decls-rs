# AST Trace: ../rust/library/compiler-builtins/crates/libm-macros/src/shared.rs

Generated 35 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=STRUCT | NAME=NestedOp | COMPLEXITY=2 | LINES=12

```rust
/* List of all functions that is shared between `libm-macros` and `libm-test`. */

use std::fmt;
use std::sync::LazyLock;

struct NestedOp {
    float_ty: FloatTy,
    rust_sig: Signature,
    c_sig: Option<Signature>,
    fn_list: &'static [&'static str],
    public: bool,
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=24

```rust
/// We need a flat list to work with most of the time, but define things as a more convenient
/// nested list.
const ALL_OPERATIONS_NESTED: &[NestedOp] = &[
    NestedOp {
        // `fn(f16) -> f16`
        float_ty: FloatTy::F16,
        rust_sig: Signature {
            args: &[Ty::F16],
            returns: &[Ty::F16],
        },
        c_sig: None,
        fn_list: &[
            "ceilf16",
            "fabsf16",
            "floorf16",
            "rintf16",
            "roundevenf16",
            "roundf16",
            "sqrtf16",
            "truncf16",
        ],
        public: true,
    },
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=5 | LINES=49

```rust
NestedOp {
        // `fn(f32) -> f32`
        float_ty: FloatTy::F32,
        rust_sig: Signature {
            args: &[Ty::F32],
            returns: &[Ty::F32],
        },
        c_sig: None,
        fn_list: &[
            "acosf",
            "acoshf",
            "asinf",
            "asinhf",
            "atanf",
            "atanhf",
            "cbrtf",
            "ceilf",
            "cosf",
            "coshf",
            "erfcf",
            "erff",
            "exp10f",
            "exp2f",
            "expf",
            "expm1f",
            "fabsf",
            "floorf",
            "j0f",
            "j1f",
            "lgammaf",
            "log10f",
            "log1pf",
            "log2f",
            "logf",
            "rintf",
            "roundevenf",
            "roundf",
            "sinf",
            "sinhf",
            "sqrtf",
            "tanf",
            "tanhf",
            "tgammaf",
            "truncf",
            "y0f",
            "y1f",
        ],
        public: true,
    },
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=5 | LINES=49

```rust
NestedOp {
        // `(f64) -> f64`
        float_ty: FloatTy::F64,
        rust_sig: Signature {
            args: &[Ty::F64],
            returns: &[Ty::F64],
        },
        c_sig: None,
        fn_list: &[
            "acos",
            "acosh",
            "asin",
            "asinh",
            "atan",
            "atanh",
            "cbrt",
            "ceil",
            "cos",
            "cosh",
            "erf",
            "erfc",
            "exp",
            "exp10",
            "exp2",
            "expm1",
            "fabs",
            "floor",
            "j0",
            "j1",
            "lgamma",
            "log",
            "log10",
            "log1p",
            "log2",
            "rint",
            "round",
            "roundeven",
            "sin",
            "sinh",
            "sqrt",
            "tan",
            "tanh",
            "tgamma",
            "trunc",
            "y0",
            "y1",
        ],
        public: true,
    },
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=20

```rust
NestedOp {
        // `fn(f128) -> f128`
        float_ty: FloatTy::F128,
        rust_sig: Signature {
            args: &[Ty::F128],
            returns: &[Ty::F128],
        },
        c_sig: None,
        fn_list: &[
            "ceilf128",
            "fabsf128",
            "floorf128",
            "rintf128",
            "roundevenf128",
            "roundf128",
            "sqrtf128",
            "truncf128",
        ],
        public: true,
    },
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=21

```rust
NestedOp {
        // `(f16, f16) -> f16`
        float_ty: FloatTy::F16,
        rust_sig: Signature {
            args: &[Ty::F16, Ty::F16],
            returns: &[Ty::F16],
        },
        c_sig: None,
        fn_list: &[
            "copysignf16",
            "fdimf16",
            "fmaxf16",
            "fmaximum_numf16",
            "fmaximumf16",
            "fminf16",
            "fminimum_numf16",
            "fminimumf16",
            "fmodf16",
        ],
        public: true,
    },
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=26

```rust
NestedOp {
        // `(f32, f32) -> f32`
        float_ty: FloatTy::F32,
        rust_sig: Signature {
            args: &[Ty::F32, Ty::F32],
            returns: &[Ty::F32],
        },
        c_sig: None,
        fn_list: &[
            "atan2f",
            "copysignf",
            "fdimf",
            "fmaxf",
            "fmaximum_numf",
            "fmaximumf",
            "fminf",
            "fminimum_numf",
            "fminimumf",
            "fmodf",
            "hypotf",
            "nextafterf",
            "powf",
            "remainderf",
        ],
        public: true,
    },
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=26

```rust
NestedOp {
        // `(f64, f64) -> f64`
        float_ty: FloatTy::F64,
        rust_sig: Signature {
            args: &[Ty::F64, Ty::F64],
            returns: &[Ty::F64],
        },
        c_sig: None,
        fn_list: &[
            "atan2",
            "copysign",
            "fdim",
            "fmax",
            "fmaximum",
            "fmaximum_num",
            "fmin",
            "fminimum",
            "fminimum_num",
            "fmod",
            "hypot",
            "nextafter",
            "pow",
            "remainder",
        ],
        public: true,
    },
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=21

```rust
NestedOp {
        // `(f128, f128) -> f128`
        float_ty: FloatTy::F128,
        rust_sig: Signature {
            args: &[Ty::F128, Ty::F128],
            returns: &[Ty::F128],
        },
        c_sig: None,
        fn_list: &[
            "copysignf128",
            "fdimf128",
            "fmaxf128",
            "fmaximum_numf128",
            "fmaximumf128",
            "fminf128",
            "fminimum_numf128",
            "fminimumf128",
            "fmodf128",
        ],
        public: true,
    },
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
NestedOp {
        // `(f32, f32, f32) -> f32`
        float_ty: FloatTy::F32,
        rust_sig: Signature {
            args: &[Ty::F32, Ty::F32, Ty::F32],
            returns: &[Ty::F32],
        },
        c_sig: None,
        fn_list: &["fmaf"],
        public: true,
    },
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
NestedOp {
        // `(f64, f64, f64) -> f64`
        float_ty: FloatTy::F64,
        rust_sig: Signature {
            args: &[Ty::F64, Ty::F64, Ty::F64],
            returns: &[Ty::F64],
        },
        c_sig: None,
        fn_list: &["fma"],
        public: true,
    },
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
NestedOp {
        // `(f128, f128, f128) -> f128`
        float_ty: FloatTy::F128,
        rust_sig: Signature {
            args: &[Ty::F128, Ty::F128, Ty::F128],
            returns: &[Ty::F128],
        },
        c_sig: None,
        fn_list: &["fmaf128"],
        public: true,
    },
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
NestedOp {
        // `(f32) -> i32`
        float_ty: FloatTy::F32,
        rust_sig: Signature {
            args: &[Ty::F32],
            returns: &[Ty::I32],
        },
        c_sig: None,
        fn_list: &["ilogbf"],
        public: true,
    },
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
NestedOp {
        // `(f64) -> i32`
        float_ty: FloatTy::F64,
        rust_sig: Signature {
            args: &[Ty::F64],
            returns: &[Ty::I32],
        },
        c_sig: None,
        fn_list: &["ilogb"],
        public: true,
    },
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
NestedOp {
        // `(i32, f32) -> f32`
        float_ty: FloatTy::F32,
        rust_sig: Signature {
            args: &[Ty::I32, Ty::F32],
            returns: &[Ty::F32],
        },
        c_sig: None,
        fn_list: &["jnf", "ynf"],
        public: true,
    },
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
NestedOp {
        // `(i32, f64) -> f64`
        float_ty: FloatTy::F64,
        rust_sig: Signature {
            args: &[Ty::I32, Ty::F64],
            returns: &[Ty::F64],
        },
        c_sig: None,
        fn_list: &["jn", "yn"],
        public: true,
    },
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
NestedOp {
        // `(f16, i32) -> f16`
        float_ty: FloatTy::F16,
        rust_sig: Signature {
            args: &[Ty::F16, Ty::I32],
            returns: &[Ty::F16],
        },
        c_sig: None,
        fn_list: &["ldexpf16", "scalbnf16"],
        public: true,
    },
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
NestedOp {
        // `(f32, i32) -> f32`
        float_ty: FloatTy::F32,
        rust_sig: Signature {
            args: &[Ty::F32, Ty::I32],
            returns: &[Ty::F32],
        },
        c_sig: None,
        fn_list: &["ldexpf", "scalbnf"],
        public: true,
    },
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
NestedOp {
        // `(f64, i64) -> f64`
        float_ty: FloatTy::F64,
        rust_sig: Signature {
            args: &[Ty::F64, Ty::I32],
            returns: &[Ty::F64],
        },
        c_sig: None,
        fn_list: &["ldexp", "scalbn"],
        public: true,
    },
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
NestedOp {
        // `(f128, i32) -> f128`
        float_ty: FloatTy::F128,
        rust_sig: Signature {
            args: &[Ty::F128, Ty::I32],
            returns: &[Ty::F128],
        },
        c_sig: None,
        fn_list: &["ldexpf128", "scalbnf128"],
        public: true,
    },
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
NestedOp {
        // `(f32, &mut f32) -> f32` as `(f32) -> (f32, f32)`
        float_ty: FloatTy::F32,
        rust_sig: Signature {
            args: &[Ty::F32],
            returns: &[Ty::F32, Ty::F32],
        },
        c_sig: Some(Signature {
            args: &[Ty::F32, Ty::MutF32],
            returns: &[Ty::F32],
        }),
        fn_list: &["modff"],
        public: true,
    },
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
NestedOp {
        // `(f64, &mut f64) -> f64` as  `(f64) -> (f64, f64)`
        float_ty: FloatTy::F64,
        rust_sig: Signature {
            args: &[Ty::F64],
            returns: &[Ty::F64, Ty::F64],
        },
        c_sig: Some(Signature {
            args: &[Ty::F64, Ty::MutF64],
            returns: &[Ty::F64],
        }),
        fn_list: &["modf"],
        public: true,
    },
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
NestedOp {
        // `(f32, &mut c_int) -> f32` as `(f32) -> (f32, i32)`
        float_ty: FloatTy::F32,
        rust_sig: Signature {
            args: &[Ty::F32],
            returns: &[Ty::F32, Ty::I32],
        },
        c_sig: Some(Signature {
            args: &[Ty::F32, Ty::MutCInt],
            returns: &[Ty::F32],
        }),
        fn_list: &["frexpf", "lgammaf_r"],
        public: true,
    },
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
NestedOp {
        // `(f64, &mut c_int) -> f64` as `(f64) -> (f64, i32)`
        float_ty: FloatTy::F64,
        rust_sig: Signature {
            args: &[Ty::F64],
            returns: &[Ty::F64, Ty::I32],
        },
        c_sig: Some(Signature {
            args: &[Ty::F64, Ty::MutCInt],
            returns: &[Ty::F64],
        }),
        fn_list: &["frexp", "lgamma_r"],
        public: true,
    },
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
NestedOp {
        // `(f32, f32, &mut c_int) -> f32` as `(f32, f32) -> (f32, i32)`
        float_ty: FloatTy::F32,
        rust_sig: Signature {
            args: &[Ty::F32, Ty::F32],
            returns: &[Ty::F32, Ty::I32],
        },
        c_sig: Some(Signature {
            args: &[Ty::F32, Ty::F32, Ty::MutCInt],
            returns: &[Ty::F32],
        }),
        fn_list: &["remquof"],
        public: true,
    },
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
NestedOp {
        // `(f64, f64, &mut c_int) -> f64` as `(f64, f64) -> (f64, i32)`
        float_ty: FloatTy::F64,
        rust_sig: Signature {
            args: &[Ty::F64, Ty::F64],
            returns: &[Ty::F64, Ty::I32],
        },
        c_sig: Some(Signature {
            args: &[Ty::F64, Ty::F64, Ty::MutCInt],
            returns: &[Ty::F64],
        }),
        fn_list: &["remquo"],
        public: true,
    },
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
NestedOp {
        // `(f32, &mut f32, &mut f32)` as `(f32) -> (f32, f32)`
        float_ty: FloatTy::F32,
        rust_sig: Signature {
            args: &[Ty::F32],
            returns: &[Ty::F32, Ty::F32],
        },
        c_sig: Some(Signature {
            args: &[Ty::F32, Ty::MutF32, Ty::MutF32],
            returns: &[],
        }),
        fn_list: &["sincosf"],
        public: true,
    },
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
NestedOp {
        // `(f64, &mut f64, &mut f64)` as `(f64) -> (f64, f64)`
        float_ty: FloatTy::F64,
        rust_sig: Signature {
            args: &[Ty::F64],
            returns: &[Ty::F64, Ty::F64],
        },
        c_sig: Some(Signature {
            args: &[Ty::F64, Ty::MutF64, Ty::MutF64],
            returns: &[],
        }),
        fn_list: &["sincos"],
        public: true,
    },
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=19

```rust
];

/// A type used in a function signature.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    F16,
    F32,
    F64,
    F128,
    I32,
    CInt,
    MutF16,
    MutF32,
    MutF64,
    MutF128,
    MutI32,
    MutCInt,
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
/// A subset of [`Ty`] representing only floats.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FloatTy {
    F16,
    F32,
    F64,
    F128,
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=10 | LINES=20

```rust
impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Ty::F16 => "f16",
            Ty::F32 => "f32",
            Ty::F64 => "f64",
            Ty::F128 => "f128",
            Ty::I32 => "i32",
            Ty::CInt => "::core::ffi::c_int",
            Ty::MutF16 => "&mut f16",
            Ty::MutF32 => "&mut f32",
            Ty::MutF64 => "&mut f64",
            Ty::MutF128 => "&mut f128",
            Ty::MutI32 => "&mut i32",
            Ty::MutCInt => "&mut ::core::ffi::c_int",
        };
        f.write_str(s)
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=9 | LINES=12

```rust
impl fmt::Display for FloatTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FloatTy::F16 => "f16",
            FloatTy::F32 => "f32",
            FloatTy::F64 => "f64",
            FloatTy::F128 => "f128",
        };
        f.write_str(s)
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=STRUCT | NAME=Signature | COMPLEXITY=2 | LINES=7

```rust
/// Representation of e.g. `(f32, f32) -> f32`
#[derive(Debug, Clone)]
pub struct Signature {
    pub args: &'static [Ty],
    pub returns: &'static [Ty],
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=STRUCT | NAME=MathOpInfo | COMPLEXITY=8 | LINES=13

```rust
/// Combined information about a function implementation.
#[derive(Debug, Clone)]
pub struct MathOpInfo {
    pub name: &'static str,
    pub float_ty: FloatTy,
    /// Function signature for C implementations
    pub c_sig: Signature,
    /// Function signature for Rust implementations
    pub rust_sig: Signature,
    /// True if part of libm's public API
    pub public: bool,
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=15 | LINES=28

```rust
/// A flat representation of `ALL_FUNCTIONS`.
pub static ALL_OPERATIONS: LazyLock<Vec<MathOpInfo>> = LazyLock::new(|| {
    let mut ret = Vec::new();

    for op in ALL_OPERATIONS_NESTED {
        let fn_names = op.fn_list;
        for name in fn_names {
            let api = MathOpInfo {
                name,
                float_ty: op.float_ty,
                rust_sig: op.rust_sig.clone(),
                c_sig: op.c_sig.clone().unwrap_or_else(|| op.rust_sig.clone()),
                public: op.public,
            };
            ret.push(api);
        }

        if !fn_names.is_sorted() {
            let mut sorted = (*fn_names).to_owned();
            sorted.sort_unstable();
            panic!("names list is not sorted: {fn_names:?}\nExpected: {sorted:?}");
        }
    }

    ret.sort_by_key(|item| item.name);
    ret
});
```

---
*Generated by AST tracing system*
