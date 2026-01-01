# AST Trace: ../rust/library/compiler-builtins/builtins-test/benches/float_cmp.rs

Generated 11 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
#![cfg_attr(f128_enabled, feature(f128))]

use builtins_test::float_bench;
use compiler_builtins::float::cmp::{self, CmpResult};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use criterion::{Criterion, criterion_main};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=gt_res_eq | COMPLEXITY=8 | LINES=19

```rust
/// `gt` symbols are allowed to return differing results, they just get compared
/// to 0.
fn gt_res_eq(mut a: CmpResult, mut b: CmpResult) -> bool {
    // FIXME: Our CmpResult used to be `i32`, but GCC/LLVM expect `isize`. on 64-bit platforms,
    // this means the top half of the word may be garbage if built with an old version of
    // `compiler-builtins`, so add a hack around this.
    //
    // This can be removed once a version of `compiler-builtins` with the return type fix makes
    // it upstream.
    if size_of::<CmpResult>() == 8 {
        a = a as i32 as CmpResult;
        b = b as i32 as CmpResult;
    }

    let a_lt_0 = a <= 0;
    let b_lt_0 = b <= 0;
    (a_lt_0 && b_lt_0) || (!a_lt_0 && !b_lt_0)
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=13 | LINES=39

```rust
float_bench! {
    name: cmp_f32_gt,
    sig: (a: f32, b: f32) -> CmpResult,
    crate_fn: cmp::__gtsf2,
    sys_fn: __gtsf2,
    sys_available: all(),
    output_eq: gt_res_eq,
    asm: [
        #[cfg(target_arch = "x86_64")] {
            let ret: CmpResult;
            asm!(
                "xor     {ret:e}, {ret:e}",
                "ucomiss {a}, {b}",
                "seta    {ret:l}",
                a = in(xmm_reg) a,
                b = in(xmm_reg) b,
                ret = out(reg) ret,
                options(nomem, nostack, pure)
            );

            ret
        };

        #[cfg(target_arch = "aarch64")] {
            let ret: CmpResult;
            asm!(
                "fcmp    {a:s}, {b:s}",
                "cset    {ret:w}, gt",
                a = in(vreg) a,
                b = in(vreg) b,
                ret = out(reg) ret,
                options(nomem,nostack),
            );

            ret
        };
    ],
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=13 | LINES=38

```rust
float_bench! {
    name: cmp_f32_unord,
    sig: (a: f32, b: f32) -> CmpResult,
    crate_fn: cmp::__unordsf2,
    sys_fn: __unordsf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")] {
            let ret: CmpResult;
            asm!(
                "xor     {ret:e}, {ret:e}",
                "ucomiss {a}, {b}",
                "setp    {ret:l}",
                a = in(xmm_reg) a,
                b = in(xmm_reg) b,
                ret = out(reg) ret,
                options(nomem, nostack, pure)
            );

            ret
        };

        #[cfg(target_arch = "aarch64")] {
            let ret: CmpResult;
            asm!(
                "fcmp    {a:s}, {b:s}",
                "cset    {ret:w}, vs",
                a = in(vreg) a,
                b = in(vreg) b,
                ret = out(reg) ret,
                options(nomem, nostack, pure)
            );

            ret
        };
    ],
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=13 | LINES=39

```rust
float_bench! {
    name: cmp_f64_gt,
    sig: (a: f64, b: f64) -> CmpResult,
    crate_fn: cmp::__gtdf2,
    sys_fn: __gtdf2,
    sys_available: all(),
    output_eq: gt_res_eq,
    asm: [
        #[cfg(target_arch = "x86_64")] {
            let ret: CmpResult;
            asm!(
                "xor     {ret:e}, {ret:e}",
                "ucomisd {a}, {b}",
                "seta    {ret:l}",
                a = in(xmm_reg) a,
                b = in(xmm_reg) b,
                ret = out(reg) ret,
                options(nomem, nostack, pure)
            );

            ret
        };

        #[cfg(target_arch = "aarch64")] {
            let ret: CmpResult;
            asm!(
                "fcmp    {a:d}, {b:d}",
                "cset {ret:w}, gt",
                a = in(vreg) a,
                b = in(vreg) b,
                ret = out(reg) ret,
                options(nomem, nostack, pure)
            );

            ret
        };
    ],
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=13 | LINES=38

```rust
float_bench! {
    name: cmp_f64_unord,
    sig: (a: f64, b: f64) -> CmpResult,
    crate_fn: cmp::__unorddf2,
    sys_fn: __unorddf2,
    sys_available: all(),
    asm: [
        #[cfg(target_arch = "x86_64")] {
            let ret: CmpResult;
            asm!(
                "xor     {ret:e}, {ret:e}",
                "ucomisd {a}, {b}",
                "setp    {ret:l}",
                a = in(xmm_reg) a,
                b = in(xmm_reg) b,
                ret = out(reg) ret,
                options(nomem, nostack, pure)
            );

            ret
        };

        #[cfg(target_arch = "aarch64")] {
            let ret: CmpResult;
            asm!(
                "fcmp    {a:d}, {b:d}",
                "cset    {ret:w}, vs",
                a = in(vreg) a,
                b = in(vreg) b,
                ret = out(reg) ret,
                options(nomem, nostack, pure)
            );

            ret
        };
    ],
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=13

```rust
#[cfg(f128_enabled)]
float_bench! {
    name: cmp_f128_gt,
    sig: (a: f128, b: f128) -> CmpResult,
    crate_fn: cmp::__gttf2,
    crate_fn_ppc: cmp::__gtkf2,
    sys_fn: __gttf2,
    sys_fn_ppc: __gtkf2,
    sys_available: not(feature = "no-sys-f128"),
    output_eq: gt_res_eq,
    asm: []
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[cfg(f128_enabled)]
float_bench! {
    name: cmp_f128_unord,
    sig: (a: f128, b: f128) -> CmpResult,
    crate_fn: cmp::__unordtf2,
    crate_fn_ppc: cmp::__unordkf2,
    sys_fn: __unordtf2,
    sys_fn_ppc: __unordkf2,
    sys_available: not(feature = "no-sys-f128"),
    asm: []
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=float_cmp | COMPLEXITY=3 | LINES=15

```rust
pub fn float_cmp() {
    let mut criterion = Criterion::default().configure_from_args();

    cmp_f32_gt(&mut criterion);
    cmp_f32_unord(&mut criterion);
    cmp_f64_gt(&mut criterion);
    cmp_f64_unord(&mut criterion);

    #[cfg(f128_enabled)]
    {
        cmp_f128_gt(&mut criterion);
        cmp_f128_unord(&mut criterion);
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=1 | LINES=2

```rust
criterion_main!(float_cmp);
```

---
*Generated by AST tracing system*
