# AST Trace: ../rust/library/compiler-builtins/compiler-builtins/src/int/sdiv.rs

Generated 6 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=29 | LINES=45

```rust
use crate::int::udiv::*;

macro_rules! sdivmod {
    (
        $unsigned_fn:ident, // name of the unsigned division function
        $signed_fn:ident, // name of the signed division function
        $uX:ident, // unsigned integer type for the inputs and outputs of `$unsigned_name`
        $iX:ident, // signed integer type for the inputs and outputs of `$signed_name`
        $($attr:tt),* // attributes
    ) => {
        intrinsics! {
            $(
                #[$attr]
            )*
            /// Returns `n / d` and sets `*rem = n % d`
            pub extern "C" fn $signed_fn(a: $iX, b: $iX, rem: &mut $iX) -> $iX {
                let a_neg = a < 0;
                let b_neg = b < 0;
                let mut a = a;
                let mut b = b;

                if a_neg {
                    a = a.wrapping_neg();
                }
                if b_neg {
                    b = b.wrapping_neg();
                }

                let mut r = *rem as $uX;
                let t = $unsigned_fn(a as $uX, b as $uX, Some(&mut r)) as $iX;
                let mut r = r as $iX;

                if a_neg {
                    r = r.wrapping_neg();
                }
                *rem = r;
                if a_neg != b_neg {
                    t.wrapping_neg()
                } else {
                    t
                }
            }
        }
    }
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=26 | LINES=35

```rust
macro_rules! sdiv {
    (
        $unsigned_fn:ident, // name of the unsigned division function
        $signed_fn:ident, // name of the signed division function
        $uX:ident, // unsigned integer type for the inputs and outputs of `$unsigned_name`
        $iX:ident, // signed integer type for the inputs and outputs of `$signed_name`
        $($attr:tt),* // attributes
    ) => {
        intrinsics! {
            $(
                #[$attr]
            )*
            /// Returns `n / d`
            pub extern "C" fn $signed_fn(a: $iX, b: $iX) -> $iX {
                let a_neg = a < 0;
                let b_neg = b < 0;
                let mut a = a;
                let mut b = b;
                if a_neg {
                    a = a.wrapping_neg();
                }
                if b_neg {
                    b = b.wrapping_neg();
                }
                let t = $unsigned_fn(a as $uX, b as $uX) as $iX;
                if a_neg != b_neg {
                    t.wrapping_neg()
                } else {
                    t
                }
            }
        }
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=26 | LINES=35

```rust
macro_rules! smod {
    (
        $unsigned_fn:ident, // name of the unsigned division function
        $signed_fn:ident, // name of the signed division function
        $uX:ident, // unsigned integer type for the inputs and outputs of `$unsigned_name`
        $iX:ident, // signed integer type for the inputs and outputs of `$signed_name`
        $($attr:tt),* // attributes
    ) => {
        intrinsics! {
            $(
                #[$attr]
            )*
            /// Returns `n % d`
            pub extern "C" fn $signed_fn(a: $iX, b: $iX) -> $iX {
                let a_neg = a < 0;
                let b_neg = b < 0;
                let mut a = a;
                let mut b = b;
                if a_neg {
                    a = a.wrapping_neg();
                }
                if b_neg {
                    b = b.wrapping_neg();
                }
                let r = $unsigned_fn(a as $uX, b as $uX) as $iX;
                if a_neg {
                    r.wrapping_neg()
                } else {
                    r
                }
            }
        }
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=19 | LINES=43

```rust
#[cfg(not(target_arch = "avr"))]
sdivmod!(
    __udivmodsi4,
    __divmodsi4,
    u32,
    i32,
    maybe_use_optimized_c_shim
);

#[cfg(target_arch = "avr")]
intrinsics! {
    /// Returns `a / b` and `a % b` packed together.
    ///
    /// Ideally we'd use `-> (u32, u32)` or some kind of a packed struct, but
    /// both force a stack allocation, while our result has to be in R18:R26.
    pub extern "C" fn __divmodsi4(a: i32, b: i32) -> u64 {
        let a_neg = a < 0;
        let b_neg = b < 0;
        let mut a = a;
        let mut b = b;

        if a_neg {
            a = a.wrapping_neg();
        }
        if b_neg {
            b = b.wrapping_neg();
        }

        let tr = __udivmodsi4(a as u32, b as u32);
        let mut t = tr as u32 as i32;
        let mut r = (tr >> 32) as u32 as i32;

        if a_neg {
            r = r.wrapping_neg();
        }
        if a_neg != b_neg {
            t = t.wrapping_neg();
        }

        ((r as u32 as u64) << 32) | (t as u32 as u64)
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=25

```rust
// The `#[arm_aeabi_alias = __aeabi_idiv]` attribute cannot be made to work with `intrinsics!` in macros
intrinsics! {
    #[maybe_use_optimized_c_shim]
    #[arm_aeabi_alias = __aeabi_idiv]
    /// Returns `n / d`
    pub extern "C" fn __divsi3(a: i32, b: i32) -> i32 {
        let a_neg = a < 0;
        let b_neg = b < 0;
        let mut a = a;
        let mut b = b;
        if a_neg {
            a = a.wrapping_neg();
        }
        if b_neg {
            b = b.wrapping_neg();
        }
        let t = __udivsi3(a as u32, b as u32) as i32;
        if a_neg != b_neg {
            t.wrapping_neg()
        } else {
            t
        }
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=22

```rust
smod!(__umodsi3, __modsi3, u32, i32, maybe_use_optimized_c_shim);

sdivmod!(
    __udivmoddi4,
    __divmoddi4,
    u64,
    i64,
    maybe_use_optimized_c_shim
);
sdiv!(__udivdi3, __divdi3, u64, i64, maybe_use_optimized_c_shim);
smod!(__umoddi3, __moddi3, u64, i64, maybe_use_optimized_c_shim);

// LLVM does not currently have a `__divmodti4` function, but GCC does
sdivmod!(
    __udivmodti4,
    __divmodti4,
    u128,
    i128,
    maybe_use_optimized_c_shim
);
sdiv!(__udivti3, __divti3, u128, i128,);
smod!(__umodti3, __modti3, u128, i128,);
```

---
*Generated by AST tracing system*
