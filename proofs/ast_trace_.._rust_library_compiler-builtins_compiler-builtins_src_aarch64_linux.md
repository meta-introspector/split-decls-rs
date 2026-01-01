# AST Trace: ../rust/library/compiler-builtins/compiler-builtins/src/aarch64_linux.rs

Generated 34 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=13 | LINES=18

```rust
//! Aarch64 targets have two possible implementations for atomics:
//! 1. Load-Locked, Store-Conditional (LL/SC), older and slower.
//! 2. Large System Extensions (LSE), newer and faster.
//! To avoid breaking backwards compat, C toolchains introduced a concept of "outlined atomics",
//! where atomic operations call into the compiler runtime to dispatch between two depending on
//! which is supported on the current CPU.
//! See <https://community.arm.com/arm-community-blogs/b/tools-software-ides-blog/posts/making-the-most-of-the-arm-architecture-in-gcc-10#:~:text=out%20of%20line%20atomics> for more discussion.
//!
//! Ported from `aarch64/lse.S` in LLVM's compiler-rt.
//!
//! Generate functions for each of the following symbols:
//!  __aarch64_casM_ORDER
//!  __aarch64_swpN_ORDER
//!  __aarch64_ldaddN_ORDER
//!  __aarch64_ldclrN_ORDER
//!  __aarch64_ldeorN_ORDER
//!  __aarch64_ldsetN_ORDER
//! for N = {1, 2, 4, 8}, M = {1, 2, 4, 8, 16}, ORDER = { relax, acq, rel, acq_rel }
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
//!
//! The original `lse.S` has some truly horrifying code that expects to be compiled multiple times with different constants.
//! We do something similar, but with macro arguments.
#![cfg_attr(feature = "c", allow(unused_macros))] // avoid putting the macros into a submodule

use core::sync::atomic::{AtomicU8, Ordering};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=11

```rust
/// non-zero if the host supports LSE atomics.
static HAVE_LSE_ATOMICS: AtomicU8 = AtomicU8::new(0);

intrinsics! {
    /// Call to enable LSE in outline atomic operations. The caller must verify
    /// LSE operations are supported.
    pub extern "C" fn __rust_enable_lse() {
        HAVE_LSE_ATOMICS.store(1, Ordering::Relaxed);
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=12 | LINES=10

```rust
/// Translate a byte size to a Rust type.
#[rustfmt::skip]
macro_rules! int_ty {
    (1) => { i8 };
    (2) => { i16 };
    (4) => { i32 };
    (8) => { i64 };
    (16) => { i128 };
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=12 | LINES=12

```rust
/// Given a byte size and a register number, return a register of the appropriate size.
///
/// See <https://developer.arm.com/documentation/102374/0101/Registers-in-AArch64---general-purpose-registers>.
#[rustfmt::skip]
macro_rules! reg {
    (1, $num:literal) => { concat!("w", $num) };
    (2, $num:literal) => { concat!("w", $num) };
    (4, $num:literal) => { concat!("w", $num) };
    (8, $num:literal) => { concat!("x", $num) };
    (16, $num:literal) => { concat!("x", $num) };
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=13 | LINES=9

```rust
/// Given an atomic ordering, translate it to the acquire suffix for the lxdr aarch64 ASM instruction.
#[rustfmt::skip]
macro_rules! acquire {
    (Relaxed) => { "" };
    (Acquire) => { "a" };
    (Release) => { "" };
    (AcqRel) => { "a" };
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=13 | LINES=9

```rust
/// Given an atomic ordering, translate it to the release suffix for the stxr aarch64 ASM instruction.
#[rustfmt::skip]
macro_rules! release {
    (Relaxed) => { "" };
    (Acquire) => { "" };
    (Release) => { "l" };
    (AcqRel) => { "l" };
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=14 | LINES=10

```rust
/// Given a size in bytes, translate it to the byte suffix for an aarch64 ASM instruction.
#[rustfmt::skip]
macro_rules! size {
    (1) => { "b" };
    (2) => { "h" };
    (4) => { "" };
    (8) => { "" };
    (16) => { "" };
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=10 | LINES=11

```rust
/// Given a byte size, translate it to an Unsigned eXTend instruction
/// with the correct semantics.
///
/// See <https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions/UXTB--Unsigned-Extend-Byte--an-alias-of-UBFM->
#[rustfmt::skip]
macro_rules! uxt {
    (1) => { "uxtb" };
    (2) => { "uxth" };
    ($_:tt) => { "mov" };
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=10

```rust
/// Given an atomic ordering and byte size, translate it to a LoaD eXclusive Register instruction
/// with the correct semantics.
///
/// See <https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions/LDXR--Load-Exclusive-Register->.
macro_rules! ldxr {
    ($ordering:ident, $bytes:tt) => {
        concat!("ld", acquire!($ordering), "xr", size!($bytes))
    };
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=10

```rust
/// Given an atomic ordering and byte size, translate it to a STore eXclusive Register instruction
/// with the correct semantics.
///
/// See <https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions/STXR--Store-Exclusive-Register->.
macro_rules! stxr {
    ($ordering:ident, $bytes:tt) => {
        concat!("st", release!($ordering), "xr", size!($bytes))
    };
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=10

```rust
/// Given an atomic ordering and byte size, translate it to a LoaD eXclusive Pair of registers instruction
/// with the correct semantics.
///
/// See <https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions/LDXP--Load-Exclusive-Pair-of-Registers->
macro_rules! ldxp {
    ($ordering:ident) => {
        concat!("ld", acquire!($ordering), "xp")
    };
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=10

```rust
/// Given an atomic ordering and byte size, translate it to a STore eXclusive Pair of registers instruction
/// with the correct semantics.
///
/// See <https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions/STXP--Store-Exclusive-Pair-of-registers->.
macro_rules! stxp {
    ($ordering:ident) => {
        concat!("st", release!($ordering), "xp")
    };
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=11 | LINES=16

```rust
// If supported, perform the requested LSE op and return, or fallthrough.
macro_rules! try_lse_op {
    ($op: literal, $ordering:ident, $bytes:tt, $($reg:literal,)* [ $mem:ident ] ) => {
        concat!(
            ".arch_extension lse; ",
            "adrp    x16, {have_lse}; ",
            "ldrb    w16, [x16, :lo12:{have_lse}]; ",
            "cbz     w16, 8f; ",
            // LSE_OP  s(reg),* [$mem]
            concat!(lse!($op, $ordering, $bytes), $( " ", reg!($bytes, $reg), ", " ,)* "[", stringify!($mem), "]; ",),
            "ret; ",
            "8:"
        )
    };
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=11 | LINES=9

```rust
// Translate memory ordering to the LSE suffix
#[rustfmt::skip]
macro_rules! lse_mem_sfx {
    (Relaxed) => { "" };
    (Acquire) => { "a" };
    (Release) => { "l" };
    (AcqRel) => { "al" };
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=11 | LINES=10

```rust
// Generate the aarch64 LSE operation for memory ordering and width
macro_rules! lse {
    ($op:literal, $order:ident, 16) => {
        concat!($op, "p", lse_mem_sfx!($order))
    };
    ($op:literal, $order:ident, $bytes:tt) => {
        concat!($op, lse_mem_sfx!($order), size!($bytes))
    };
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=20 | LINES=33

```rust
/// See <https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicI8.html#method.compare_and_swap>.
macro_rules! compare_and_swap {
    ($ordering:ident, $bytes:tt, $name:ident) => {
        intrinsics! {
            #[maybe_use_optimized_c_shim]
            #[unsafe(naked)]
            pub unsafe extern "C" fn $name (
                expected: int_ty!($bytes), desired: int_ty!($bytes), ptr: *mut int_ty!($bytes)
            ) -> int_ty!($bytes) {
                // We can't use `AtomicI8::compare_and_swap`; we *are* compare_and_swap.
                core::arch::naked_asm! {
                    // CAS    s(0), s(1), [x2]; if LSE supported.
                    try_lse_op!("cas", $ordering, $bytes, 0, 1, [x2]),
                    // UXT    s(tmp0), s(0)
                    concat!(uxt!($bytes), " ", reg!($bytes, 16), ", ", reg!($bytes, 0)),
                    "0:",
                    // LDXR   s(0), [x2]
                    concat!(ldxr!($ordering, $bytes), " ", reg!($bytes, 0), ", [x2]"),
                    // cmp    s(0), s(tmp0)
                    concat!("cmp ", reg!($bytes, 0), ", ", reg!($bytes, 16)),
                    "bne    1f",
                    // STXR   w(tmp1), s(1), [x2]
                    concat!(stxr!($ordering, $bytes), " w17, ", reg!($bytes, 1), ", [x2]"),
                    "cbnz   w17, 0b",
                    "1:",
                    "ret",
                    have_lse = sym crate::aarch64_linux::HAVE_LSE_ATOMICS,
                }
            }
        }
    };
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=19 | LINES=32

```rust
// i128 uses a completely different impl, so it has its own macro.
macro_rules! compare_and_swap_i128 {
    ($ordering:ident, $name:ident) => {
        intrinsics! {
            #[maybe_use_optimized_c_shim]
            #[unsafe(naked)]
            pub unsafe extern "C" fn $name (
                expected: i128, desired: i128, ptr: *mut i128
            ) -> i128 {
                core::arch::naked_asm! {
                    // CASP   x0, x1, x2, x3, [x4]; if LSE supported.
                    try_lse_op!("cas", $ordering, 16, 0, 1, 2, 3, [x4]),
                    "mov    x16, x0",
                    "mov    x17, x1",
                    "0:",
                    // LDXP   x0, x1, [x4]
                    concat!(ldxp!($ordering), " x0, x1, [x4]"),
                    "cmp    x0, x16",
                    "ccmp   x1, x17, #0, eq",
                    "bne    1f",
                    // STXP   w(tmp2), x2, x3, [x4]
                    concat!(stxp!($ordering), " w15, x2, x3, [x4]"),
                    "cbnz   w15, 0b",
                    "1:",
                    "ret",
                    have_lse = sym crate::aarch64_linux::HAVE_LSE_ATOMICS,
                }
            }
        }
    };
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=19 | LINES=28

```rust
/// See <https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicI8.html#method.swap>.
macro_rules! swap {
    ($ordering:ident, $bytes:tt, $name:ident) => {
        intrinsics! {
            #[maybe_use_optimized_c_shim]
            #[unsafe(naked)]
            pub unsafe extern "C" fn $name (
                left: int_ty!($bytes), right_ptr: *mut int_ty!($bytes)
            ) -> int_ty!($bytes) {
                core::arch::naked_asm! {
                    // SWP    s(0), s(0), [x1]; if LSE supported.
                    try_lse_op!("swp", $ordering, $bytes, 0, 0, [x1]),
                    // mov    s(tmp0), s(0)
                    concat!("mov ", reg!($bytes, 16), ", ", reg!($bytes, 0)),
                    "0:",
                    // LDXR   s(0), [x1]
                    concat!(ldxr!($ordering, $bytes), " ", reg!($bytes, 0), ", [x1]"),
                    // STXR   w(tmp1), s(tmp0), [x1]
                    concat!(stxr!($ordering, $bytes), " w17, ", reg!($bytes, 16), ", [x1]"),
                    "cbnz   w17, 0b",
                    "ret",
                    have_lse = sym crate::aarch64_linux::HAVE_LSE_ATOMICS,
                }
            }
        }
    };
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=19 | LINES=30

```rust
/// See (e.g.) <https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicI8.html#method.fetch_add>.
macro_rules! fetch_op {
    ($ordering:ident, $bytes:tt, $name:ident, $op:literal, $lse_op:literal) => {
        intrinsics! {
            #[maybe_use_optimized_c_shim]
            #[unsafe(naked)]
            pub unsafe extern "C" fn $name (
                val: int_ty!($bytes), ptr: *mut int_ty!($bytes)
            ) -> int_ty!($bytes) {
                core::arch::naked_asm! {
                    // LSEOP  s(0), s(0), [x1]; if LSE supported.
                    try_lse_op!($lse_op, $ordering, $bytes, 0, 0, [x1]),
                    // mov    s(tmp0), s(0)
                    concat!("mov ", reg!($bytes, 16), ", ", reg!($bytes, 0)),
                    "0:",
                    // LDXR   s(0), [x1]
                    concat!(ldxr!($ordering, $bytes), " ", reg!($bytes, 0), ", [x1]"),
                    // OP     s(tmp1), s(0), s(tmp0)
                    concat!($op, " ", reg!($bytes, 17), ", ", reg!($bytes, 0), ", ", reg!($bytes, 16)),
                    // STXR   w(tmp2), s(tmp1), [x1]
                    concat!(stxr!($ordering, $bytes), " w15, ", reg!($bytes, 17), ", [x1]"),
                    "cbnz  w15, 0b",
                    "ret",
                    have_lse = sym crate::aarch64_linux::HAVE_LSE_ATOMICS,
                }
            }
        }
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=9 | LINES=7

```rust
// We need a single macro to pass to `foreach_ldadd`.
macro_rules! add {
    ($ordering:ident, $bytes:tt, $name:ident) => {
        fetch_op! { $ordering, $bytes, $name, "add", "ldadd" }
    };
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=9 | LINES=6

```rust
macro_rules! and {
    ($ordering:ident, $bytes:tt, $name:ident) => {
        fetch_op! { $ordering, $bytes, $name, "bic", "ldclr" }
    };
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=9 | LINES=6

```rust
macro_rules! xor {
    ($ordering:ident, $bytes:tt, $name:ident) => {
        fetch_op! { $ordering, $bytes, $name, "eor", "ldeor" }
    };
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=9 | LINES=6

```rust
macro_rules! or {
    ($ordering:ident, $bytes:tt, $name:ident) => {
        fetch_op! { $ordering, $bytes, $name, "orr", "ldset" }
    };
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=18 | LINES=16

```rust
#[macro_export]
macro_rules! foreach_ordering {
    ($macro:path, $bytes:tt, $name:ident) => {
        $macro!( Relaxed, $bytes, ${concat($name, _relax)} );
        $macro!( Acquire, $bytes, ${concat($name, _acq)} );
        $macro!( Release, $bytes, ${concat($name, _rel)} );
        $macro!( AcqRel, $bytes, ${concat($name, _acq_rel)} );
    };
    ($macro:path, $name:ident) => {
        $macro!( Relaxed, ${concat($name, _relax)} );
        $macro!( Acquire, ${concat($name, _acq)} );
        $macro!( Release, ${concat($name, _rel)} );
        $macro!( AcqRel, ${concat($name, _acq_rel)} );
    };
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=12 | LINES=10

```rust
#[macro_export]
macro_rules! foreach_bytes {
    ($macro:path, $name:ident) => {
        foreach_ordering!( $macro, 1, ${concat(__aarch64_, $name, "1")} );
        foreach_ordering!( $macro, 2, ${concat(__aarch64_, $name, "2")} );
        foreach_ordering!( $macro, 4, ${concat(__aarch64_, $name, "4")} );
        foreach_ordering!( $macro, 8, ${concat(__aarch64_, $name, "8")} );
    };
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=10 | LINES=8

```rust
/// Generate different macros for cas/swp/add/clr/eor/set so that we can test them separately.
#[macro_export]
macro_rules! foreach_cas {
    ($macro:path) => {
        foreach_bytes!($macro, cas);
    };
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=8

```rust
/// Only CAS supports 16 bytes, and it has a different implementation that uses a different macro.
#[macro_export]
macro_rules! foreach_cas16 {
    ($macro:path) => {
        foreach_ordering!($macro, __aarch64_cas16);
    };
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[macro_export]
macro_rules! foreach_swp {
    ($macro:path) => {
        foreach_bytes!($macro, swp);
    };
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[macro_export]
macro_rules! foreach_ldadd {
    ($macro:path) => {
        foreach_bytes!($macro, ldadd);
    };
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[macro_export]
macro_rules! foreach_ldclr {
    ($macro:path) => {
        foreach_bytes!($macro, ldclr);
    };
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[macro_export]
macro_rules! foreach_ldeor {
    ($macro:path) => {
        foreach_bytes!($macro, ldeor);
    };
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[macro_export]
macro_rules! foreach_ldset {
    ($macro:path) => {
        foreach_bytes!($macro, ldset);
    };
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=1 | LINES=8

```rust
foreach_cas!(compare_and_swap);
foreach_cas16!(compare_and_swap_i128);
foreach_swp!(swap);
foreach_ldadd!(add);
foreach_ldclr!(and);
foreach_ldeor!(xor);
foreach_ldset!(or);
```

---
*Generated by AST tracing system*
