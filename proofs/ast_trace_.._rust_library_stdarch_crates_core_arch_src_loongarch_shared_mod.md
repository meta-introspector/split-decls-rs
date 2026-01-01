# AST Trace: ../rust/library/stdarch/crates/core_arch/src/loongarch_shared/mod.rs

Generated 26 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=FUNCTION | NAME=rdtimel_w | COMPLEXITY=9 | LINES=12

```rust
//! `Shared LoongArch` intrinsics

use crate::arch::asm;

/// Reads the lower 32-bit stable counter value and the counter ID
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn rdtimel_w() -> (i32, isize) {
    let (val, tid): (i32, isize);
    unsafe { asm!("rdtimel.w {}, {}", out(reg) val, out(reg) tid, options(readonly, nostack)) };
    (val, tid)
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=rdtimeh_w | COMPLEXITY=9 | LINES=9

```rust
/// Reads the upper 32-bit stable counter value and the counter ID
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn rdtimeh_w() -> (i32, isize) {
    let (val, tid): (i32, isize);
    unsafe { asm!("rdtimeh.w {}, {}", out(reg) val, out(reg) tid, options(readonly, nostack)) };
    (val, tid)
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=__crc_w_b_w | COMPLEXITY=9 | LINES=50

```rust
#[allow(improper_ctypes)]
unsafe extern "unadjusted" {
    #[link_name = "llvm.loongarch.crc.w.b.w"]
    fn __crc_w_b_w(a: i32, b: i32) -> i32;
    #[link_name = "llvm.loongarch.crc.w.h.w"]
    fn __crc_w_h_w(a: i32, b: i32) -> i32;
    #[link_name = "llvm.loongarch.crc.w.w.w"]
    fn __crc_w_w_w(a: i32, b: i32) -> i32;
    #[link_name = "llvm.loongarch.crcc.w.b.w"]
    fn __crcc_w_b_w(a: i32, b: i32) -> i32;
    #[link_name = "llvm.loongarch.crcc.w.h.w"]
    fn __crcc_w_h_w(a: i32, b: i32) -> i32;
    #[link_name = "llvm.loongarch.crcc.w.w.w"]
    fn __crcc_w_w_w(a: i32, b: i32) -> i32;
    #[link_name = "llvm.loongarch.dbar"]
    fn __dbar(a: i32);
    #[link_name = "llvm.loongarch.ibar"]
    fn __ibar(a: i32);
    #[link_name = "llvm.loongarch.movgr2fcsr"]
    fn __movgr2fcsr(a: i32, b: i32);
    #[link_name = "llvm.loongarch.movfcsr2gr"]
    fn __movfcsr2gr(a: i32) -> i32;
    #[link_name = "llvm.loongarch.iocsrrd.b"]
    fn __iocsrrd_b(a: i32) -> i32;
    #[link_name = "llvm.loongarch.iocsrrd.h"]
    fn __iocsrrd_h(a: i32) -> i32;
    #[link_name = "llvm.loongarch.iocsrrd.w"]
    fn __iocsrrd_w(a: i32) -> i32;
    #[link_name = "llvm.loongarch.iocsrwr.b"]
    fn __iocsrwr_b(a: i32, b: i32);
    #[link_name = "llvm.loongarch.iocsrwr.h"]
    fn __iocsrwr_h(a: i32, b: i32);
    #[link_name = "llvm.loongarch.iocsrwr.w"]
    fn __iocsrwr_w(a: i32, b: i32);
    #[link_name = "llvm.loongarch.break"]
    fn __break(a: i32);
    #[link_name = "llvm.loongarch.cpucfg"]
    fn __cpucfg(a: i32) -> i32;
    #[link_name = "llvm.loongarch.syscall"]
    fn __syscall(a: i32);
    #[link_name = "llvm.loongarch.frecipe.s"]
    fn __frecipe_s(a: f32) -> f32;
    #[link_name = "llvm.loongarch.frecipe.d"]
    fn __frecipe_d(a: f64) -> f64;
    #[link_name = "llvm.loongarch.frsqrte.s"]
    fn __frsqrte_s(a: f32) -> f32;
    #[link_name = "llvm.loongarch.frsqrte.d"]
    fn __frsqrte_d(a: f64) -> f64;
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=crc_w_b_w | COMPLEXITY=7 | LINES=7

```rust
/// Calculate the CRC value using the IEEE 802.3 polynomial (0xEDB88320)
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn crc_w_b_w(a: i32, b: i32) -> i32 {
    unsafe { __crc_w_b_w(a, b) }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=crc_w_h_w | COMPLEXITY=7 | LINES=7

```rust
/// Calculate the CRC value using the IEEE 802.3 polynomial (0xEDB88320)
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn crc_w_h_w(a: i32, b: i32) -> i32 {
    unsafe { __crc_w_h_w(a, b) }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=crc_w_w_w | COMPLEXITY=7 | LINES=7

```rust
/// Calculate the CRC value using the IEEE 802.3 polynomial (0xEDB88320)
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn crc_w_w_w(a: i32, b: i32) -> i32 {
    unsafe { __crc_w_w_w(a, b) }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=crcc_w_b_w | COMPLEXITY=7 | LINES=7

```rust
/// Calculate the CRC value using the Castagnoli polynomial (0x82F63B78)
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn crcc_w_b_w(a: i32, b: i32) -> i32 {
    unsafe { __crcc_w_b_w(a, b) }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=crcc_w_h_w | COMPLEXITY=7 | LINES=7

```rust
/// Calculate the CRC value using the Castagnoli polynomial (0x82F63B78)
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn crcc_w_h_w(a: i32, b: i32) -> i32 {
    unsafe { __crcc_w_h_w(a, b) }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=crcc_w_w_w | COMPLEXITY=7 | LINES=7

```rust
/// Calculate the CRC value using the Castagnoli polynomial (0x82F63B78)
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn crcc_w_w_w(a: i32, b: i32) -> i32 {
    unsafe { __crcc_w_w_w(a, b) }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=dbar | COMPLEXITY=7 | LINES=8

```rust
/// Generates the memory barrier instruction
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn dbar<const IMM15: i32>() {
    static_assert_uimm_bits!(IMM15, 15);
    unsafe { __dbar(IMM15) };
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=ibar | COMPLEXITY=7 | LINES=8

```rust
/// Generates the instruction-fetch barrier instruction
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn ibar<const IMM15: i32>() {
    static_assert_uimm_bits!(IMM15, 15);
    unsafe { __ibar(IMM15) };
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=8

```rust
/// Moves data from a GPR to the FCSR
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn movgr2fcsr<const IMM5: i32>(a: i32) {
    static_assert_uimm_bits!(IMM5, 5);
    __movgr2fcsr(IMM5, a);
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=movfcsr2gr | COMPLEXITY=7 | LINES=8

```rust
/// Moves data from a FCSR to the GPR
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn movfcsr2gr<const IMM5: i32>() -> i32 {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { __movfcsr2gr(IMM5) }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=7

```rust
/// Reads the 8-bit IO-CSR
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn iocsrrd_b(a: i32) -> i32 {
    __iocsrrd_b(a)
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=7

```rust
/// Reads the 16-bit IO-CSR
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn iocsrrd_h(a: i32) -> i32 {
    __iocsrrd_h(a)
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=7

```rust
/// Reads the 32-bit IO-CSR
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn iocsrrd_w(a: i32) -> i32 {
    __iocsrrd_w(a)
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=7

```rust
/// Writes the 8-bit IO-CSR
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn iocsrwr_b(a: i32, b: i32) {
    __iocsrwr_b(a, b)
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=7

```rust
/// Writes the 16-bit IO-CSR
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn iocsrwr_h(a: i32, b: i32) {
    __iocsrwr_h(a, b)
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=7

```rust
/// Writes the 32-bit IO-CSR
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn iocsrwr_w(a: i32, b: i32) {
    __iocsrwr_w(a, b)
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=8

```rust
/// Generates the breakpoint instruction
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn brk<const IMM15: i32>() {
    static_assert_uimm_bits!(IMM15, 15);
    __break(IMM15);
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=cpucfg | COMPLEXITY=7 | LINES=7

```rust
/// Reads the CPU configuration register
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn cpucfg(a: i32) -> i32 {
    unsafe { __cpucfg(a) }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=8

```rust
/// Generates the syscall instruction
#[inline]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn syscall<const IMM15: i32>() {
    static_assert_uimm_bits!(IMM15, 15);
    __syscall(IMM15);
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=frecipe_s | COMPLEXITY=7 | LINES=8

```rust
/// Calculate the approximate single-precision result of 1.0 divided
#[inline]
#[target_feature(enable = "frecipe")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn frecipe_s(a: f32) -> f32 {
    unsafe { __frecipe_s(a) }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=frecipe_d | COMPLEXITY=7 | LINES=8

```rust
/// Calculate the approximate double-precision result of 1.0 divided
#[inline]
#[target_feature(enable = "frecipe")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn frecipe_d(a: f64) -> f64 {
    unsafe { __frecipe_d(a) }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=frsqrte_s | COMPLEXITY=7 | LINES=8

```rust
/// Calculate the approximate single-precision result of dividing 1.0 by the square root
#[inline]
#[target_feature(enable = "frecipe")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn frsqrte_s(a: f32) -> f32 {
    unsafe { __frsqrte_s(a) }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=frsqrte_d | COMPLEXITY=7 | LINES=8

```rust
/// Calculate the approximate double-precision result of dividing 1.0 by the square root
#[inline]
#[target_feature(enable = "frecipe")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn frsqrte_d(a: f64) -> f64 {
    unsafe { __frsqrte_d(a) }
}
```

---
*Generated by AST tracing system*
