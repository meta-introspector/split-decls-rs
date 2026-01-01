# AST Trace: ../rust/library/stdarch/crates/core_arch/src/loongarch64/lasx/generated.rs

Generated 741 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=FUNCTION | NAME=__lasx_xvsll_b | COMPLEXITY=173 | LINES=1494

```rust
// This code is automatically generated. DO NOT MODIFY.
//
// Instead, modify `crates/stdarch-gen-loongarch/lasx.spec` and run the following command to re-generate this file:
//
// ```
// OUT_DIR=`pwd`/crates/core_arch cargo run -p stdarch-gen-loongarch -- crates/stdarch-gen-loongarch/lasx.spec
// ```

use crate::mem::transmute;
use super::types::*;

#[allow(improper_ctypes)]
unsafe extern "unadjusted" {
    #[link_name = "llvm.loongarch.lasx.xvsll.b"]
    fn __lasx_xvsll_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsll.h"]
    fn __lasx_xvsll_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsll.w"]
    fn __lasx_xvsll_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsll.d"]
    fn __lasx_xvsll_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvslli.b"]
    fn __lasx_xvslli_b(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvslli.h"]
    fn __lasx_xvslli_h(a: __v16i16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvslli.w"]
    fn __lasx_xvslli_w(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvslli.d"]
    fn __lasx_xvslli_d(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsra.b"]
    fn __lasx_xvsra_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsra.h"]
    fn __lasx_xvsra_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsra.w"]
    fn __lasx_xvsra_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsra.d"]
    fn __lasx_xvsra_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsrai.b"]
    fn __lasx_xvsrai_b(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrai.h"]
    fn __lasx_xvsrai_h(a: __v16i16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrai.w"]
    fn __lasx_xvsrai_w(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsrai.d"]
    fn __lasx_xvsrai_d(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsrar.b"]
    fn __lasx_xvsrar_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrar.h"]
    fn __lasx_xvsrar_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrar.w"]
    fn __lasx_xvsrar_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsrar.d"]
    fn __lasx_xvsrar_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsrari.b"]
    fn __lasx_xvsrari_b(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrari.h"]
    fn __lasx_xvsrari_h(a: __v16i16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrari.w"]
    fn __lasx_xvsrari_w(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsrari.d"]
    fn __lasx_xvsrari_d(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsrl.b"]
    fn __lasx_xvsrl_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrl.h"]
    fn __lasx_xvsrl_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrl.w"]
    fn __lasx_xvsrl_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsrl.d"]
    fn __lasx_xvsrl_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsrli.b"]
    fn __lasx_xvsrli_b(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrli.h"]
    fn __lasx_xvsrli_h(a: __v16i16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrli.w"]
    fn __lasx_xvsrli_w(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsrli.d"]
    fn __lasx_xvsrli_d(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsrlr.b"]
    fn __lasx_xvsrlr_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrlr.h"]
    fn __lasx_xvsrlr_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrlr.w"]
    fn __lasx_xvsrlr_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsrlr.d"]
    fn __lasx_xvsrlr_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsrlri.b"]
    fn __lasx_xvsrlri_b(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrlri.h"]
    fn __lasx_xvsrlri_h(a: __v16i16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrlri.w"]
    fn __lasx_xvsrlri_w(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsrlri.d"]
    fn __lasx_xvsrlri_d(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvbitclr.b"]
    fn __lasx_xvbitclr_b(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvbitclr.h"]
    fn __lasx_xvbitclr_h(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvbitclr.w"]
    fn __lasx_xvbitclr_w(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvbitclr.d"]
    fn __lasx_xvbitclr_d(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvbitclri.b"]
    fn __lasx_xvbitclri_b(a: __v32u8, b: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvbitclri.h"]
    fn __lasx_xvbitclri_h(a: __v16u16, b: u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvbitclri.w"]
    fn __lasx_xvbitclri_w(a: __v8u32, b: u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvbitclri.d"]
    fn __lasx_xvbitclri_d(a: __v4u64, b: u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvbitset.b"]
    fn __lasx_xvbitset_b(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvbitset.h"]
    fn __lasx_xvbitset_h(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvbitset.w"]
    fn __lasx_xvbitset_w(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvbitset.d"]
    fn __lasx_xvbitset_d(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvbitseti.b"]
    fn __lasx_xvbitseti_b(a: __v32u8, b: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvbitseti.h"]
    fn __lasx_xvbitseti_h(a: __v16u16, b: u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvbitseti.w"]
    fn __lasx_xvbitseti_w(a: __v8u32, b: u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvbitseti.d"]
    fn __lasx_xvbitseti_d(a: __v4u64, b: u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvbitrev.b"]
    fn __lasx_xvbitrev_b(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvbitrev.h"]
    fn __lasx_xvbitrev_h(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvbitrev.w"]
    fn __lasx_xvbitrev_w(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvbitrev.d"]
    fn __lasx_xvbitrev_d(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvbitrevi.b"]
    fn __lasx_xvbitrevi_b(a: __v32u8, b: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvbitrevi.h"]
    fn __lasx_xvbitrevi_h(a: __v16u16, b: u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvbitrevi.w"]
    fn __lasx_xvbitrevi_w(a: __v8u32, b: u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvbitrevi.d"]
    fn __lasx_xvbitrevi_d(a: __v4u64, b: u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvadd.b"]
    fn __lasx_xvadd_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvadd.h"]
    fn __lasx_xvadd_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvadd.w"]
    fn __lasx_xvadd_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvadd.d"]
    fn __lasx_xvadd_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddi.bu"]
    fn __lasx_xvaddi_bu(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvaddi.hu"]
    fn __lasx_xvaddi_hu(a: __v16i16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvaddi.wu"]
    fn __lasx_xvaddi_wu(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvaddi.du"]
    fn __lasx_xvaddi_du(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsub.b"]
    fn __lasx_xvsub_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsub.h"]
    fn __lasx_xvsub_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsub.w"]
    fn __lasx_xvsub_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsub.d"]
    fn __lasx_xvsub_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsubi.bu"]
    fn __lasx_xvsubi_bu(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsubi.hu"]
    fn __lasx_xvsubi_hu(a: __v16i16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsubi.wu"]
    fn __lasx_xvsubi_wu(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsubi.du"]
    fn __lasx_xvsubi_du(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmax.b"]
    fn __lasx_xvmax_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvmax.h"]
    fn __lasx_xvmax_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmax.w"]
    fn __lasx_xvmax_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmax.d"]
    fn __lasx_xvmax_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmaxi.b"]
    fn __lasx_xvmaxi_b(a: __v32i8, b: i32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvmaxi.h"]
    fn __lasx_xvmaxi_h(a: __v16i16, b: i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmaxi.w"]
    fn __lasx_xvmaxi_w(a: __v8i32, b: i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmaxi.d"]
    fn __lasx_xvmaxi_d(a: __v4i64, b: i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmax.bu"]
    fn __lasx_xvmax_bu(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvmax.hu"]
    fn __lasx_xvmax_hu(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvmax.wu"]
    fn __lasx_xvmax_wu(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvmax.du"]
    fn __lasx_xvmax_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvmaxi.bu"]
    fn __lasx_xvmaxi_bu(a: __v32u8, b: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvmaxi.hu"]
    fn __lasx_xvmaxi_hu(a: __v16u16, b: u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvmaxi.wu"]
    fn __lasx_xvmaxi_wu(a: __v8u32, b: u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvmaxi.du"]
    fn __lasx_xvmaxi_du(a: __v4u64, b: u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvmin.b"]
    fn __lasx_xvmin_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvmin.h"]
    fn __lasx_xvmin_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmin.w"]
    fn __lasx_xvmin_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmin.d"]
    fn __lasx_xvmin_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmini.b"]
    fn __lasx_xvmini_b(a: __v32i8, b: i32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvmini.h"]
    fn __lasx_xvmini_h(a: __v16i16, b: i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmini.w"]
    fn __lasx_xvmini_w(a: __v8i32, b: i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmini.d"]
    fn __lasx_xvmini_d(a: __v4i64, b: i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmin.bu"]
    fn __lasx_xvmin_bu(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvmin.hu"]
    fn __lasx_xvmin_hu(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvmin.wu"]
    fn __lasx_xvmin_wu(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvmin.du"]
    fn __lasx_xvmin_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvmini.bu"]
    fn __lasx_xvmini_bu(a: __v32u8, b: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvmini.hu"]
    fn __lasx_xvmini_hu(a: __v16u16, b: u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvmini.wu"]
    fn __lasx_xvmini_wu(a: __v8u32, b: u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvmini.du"]
    fn __lasx_xvmini_du(a: __v4u64, b: u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvseq.b"]
    fn __lasx_xvseq_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvseq.h"]
    fn __lasx_xvseq_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvseq.w"]
    fn __lasx_xvseq_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvseq.d"]
    fn __lasx_xvseq_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvseqi.b"]
    fn __lasx_xvseqi_b(a: __v32i8, b: i32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvseqi.h"]
    fn __lasx_xvseqi_h(a: __v16i16, b: i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvseqi.w"]
    fn __lasx_xvseqi_w(a: __v8i32, b: i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvseqi.d"]
    fn __lasx_xvseqi_d(a: __v4i64, b: i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvslt.b"]
    fn __lasx_xvslt_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvslt.h"]
    fn __lasx_xvslt_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvslt.w"]
    fn __lasx_xvslt_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvslt.d"]
    fn __lasx_xvslt_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvslti.b"]
    fn __lasx_xvslti_b(a: __v32i8, b: i32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvslti.h"]
    fn __lasx_xvslti_h(a: __v16i16, b: i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvslti.w"]
    fn __lasx_xvslti_w(a: __v8i32, b: i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvslti.d"]
    fn __lasx_xvslti_d(a: __v4i64, b: i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvslt.bu"]
    fn __lasx_xvslt_bu(a: __v32u8, b: __v32u8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvslt.hu"]
    fn __lasx_xvslt_hu(a: __v16u16, b: __v16u16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvslt.wu"]
    fn __lasx_xvslt_wu(a: __v8u32, b: __v8u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvslt.du"]
    fn __lasx_xvslt_du(a: __v4u64, b: __v4u64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvslti.bu"]
    fn __lasx_xvslti_bu(a: __v32u8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvslti.hu"]
    fn __lasx_xvslti_hu(a: __v16u16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvslti.wu"]
    fn __lasx_xvslti_wu(a: __v8u32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvslti.du"]
    fn __lasx_xvslti_du(a: __v4u64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsle.b"]
    fn __lasx_xvsle_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsle.h"]
    fn __lasx_xvsle_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsle.w"]
    fn __lasx_xvsle_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsle.d"]
    fn __lasx_xvsle_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvslei.b"]
    fn __lasx_xvslei_b(a: __v32i8, b: i32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvslei.h"]
    fn __lasx_xvslei_h(a: __v16i16, b: i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvslei.w"]
    fn __lasx_xvslei_w(a: __v8i32, b: i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvslei.d"]
    fn __lasx_xvslei_d(a: __v4i64, b: i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsle.bu"]
    fn __lasx_xvsle_bu(a: __v32u8, b: __v32u8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsle.hu"]
    fn __lasx_xvsle_hu(a: __v16u16, b: __v16u16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsle.wu"]
    fn __lasx_xvsle_wu(a: __v8u32, b: __v8u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsle.du"]
    fn __lasx_xvsle_du(a: __v4u64, b: __v4u64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvslei.bu"]
    fn __lasx_xvslei_bu(a: __v32u8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvslei.hu"]
    fn __lasx_xvslei_hu(a: __v16u16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvslei.wu"]
    fn __lasx_xvslei_wu(a: __v8u32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvslei.du"]
    fn __lasx_xvslei_du(a: __v4u64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsat.b"]
    fn __lasx_xvsat_b(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsat.h"]
    fn __lasx_xvsat_h(a: __v16i16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsat.w"]
    fn __lasx_xvsat_w(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsat.d"]
    fn __lasx_xvsat_d(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsat.bu"]
    fn __lasx_xvsat_bu(a: __v32u8, b: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvsat.hu"]
    fn __lasx_xvsat_hu(a: __v16u16, b: u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvsat.wu"]
    fn __lasx_xvsat_wu(a: __v8u32, b: u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvsat.du"]
    fn __lasx_xvsat_du(a: __v4u64, b: u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvadda.b"]
    fn __lasx_xvadda_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvadda.h"]
    fn __lasx_xvadda_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvadda.w"]
    fn __lasx_xvadda_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvadda.d"]
    fn __lasx_xvadda_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsadd.b"]
    fn __lasx_xvsadd_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsadd.h"]
    fn __lasx_xvsadd_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsadd.w"]
    fn __lasx_xvsadd_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsadd.d"]
    fn __lasx_xvsadd_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsadd.bu"]
    fn __lasx_xvsadd_bu(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvsadd.hu"]
    fn __lasx_xvsadd_hu(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvsadd.wu"]
    fn __lasx_xvsadd_wu(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvsadd.du"]
    fn __lasx_xvsadd_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvavg.b"]
    fn __lasx_xvavg_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvavg.h"]
    fn __lasx_xvavg_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvavg.w"]
    fn __lasx_xvavg_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvavg.d"]
    fn __lasx_xvavg_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvavg.bu"]
    fn __lasx_xvavg_bu(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvavg.hu"]
    fn __lasx_xvavg_hu(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvavg.wu"]
    fn __lasx_xvavg_wu(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvavg.du"]
    fn __lasx_xvavg_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvavgr.b"]
    fn __lasx_xvavgr_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvavgr.h"]
    fn __lasx_xvavgr_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvavgr.w"]
    fn __lasx_xvavgr_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvavgr.d"]
    fn __lasx_xvavgr_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvavgr.bu"]
    fn __lasx_xvavgr_bu(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvavgr.hu"]
    fn __lasx_xvavgr_hu(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvavgr.wu"]
    fn __lasx_xvavgr_wu(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvavgr.du"]
    fn __lasx_xvavgr_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvssub.b"]
    fn __lasx_xvssub_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvssub.h"]
    fn __lasx_xvssub_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvssub.w"]
    fn __lasx_xvssub_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssub.d"]
    fn __lasx_xvssub_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvssub.bu"]
    fn __lasx_xvssub_bu(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvssub.hu"]
    fn __lasx_xvssub_hu(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvssub.wu"]
    fn __lasx_xvssub_wu(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvssub.du"]
    fn __lasx_xvssub_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvabsd.b"]
    fn __lasx_xvabsd_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvabsd.h"]
    fn __lasx_xvabsd_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvabsd.w"]
    fn __lasx_xvabsd_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvabsd.d"]
    fn __lasx_xvabsd_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvabsd.bu"]
    fn __lasx_xvabsd_bu(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvabsd.hu"]
    fn __lasx_xvabsd_hu(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvabsd.wu"]
    fn __lasx_xvabsd_wu(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvabsd.du"]
    fn __lasx_xvabsd_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvmul.b"]
    fn __lasx_xvmul_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvmul.h"]
    fn __lasx_xvmul_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmul.w"]
    fn __lasx_xvmul_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmul.d"]
    fn __lasx_xvmul_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmadd.b"]
    fn __lasx_xvmadd_b(a: __v32i8, b: __v32i8, c: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvmadd.h"]
    fn __lasx_xvmadd_h(a: __v16i16, b: __v16i16, c: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmadd.w"]
    fn __lasx_xvmadd_w(a: __v8i32, b: __v8i32, c: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmadd.d"]
    fn __lasx_xvmadd_d(a: __v4i64, b: __v4i64, c: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmsub.b"]
    fn __lasx_xvmsub_b(a: __v32i8, b: __v32i8, c: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvmsub.h"]
    fn __lasx_xvmsub_h(a: __v16i16, b: __v16i16, c: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmsub.w"]
    fn __lasx_xvmsub_w(a: __v8i32, b: __v8i32, c: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmsub.d"]
    fn __lasx_xvmsub_d(a: __v4i64, b: __v4i64, c: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvdiv.b"]
    fn __lasx_xvdiv_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvdiv.h"]
    fn __lasx_xvdiv_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvdiv.w"]
    fn __lasx_xvdiv_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvdiv.d"]
    fn __lasx_xvdiv_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvdiv.bu"]
    fn __lasx_xvdiv_bu(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvdiv.hu"]
    fn __lasx_xvdiv_hu(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvdiv.wu"]
    fn __lasx_xvdiv_wu(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvdiv.du"]
    fn __lasx_xvdiv_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvhaddw.h.b"]
    fn __lasx_xvhaddw_h_b(a: __v32i8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvhaddw.w.h"]
    fn __lasx_xvhaddw_w_h(a: __v16i16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvhaddw.d.w"]
    fn __lasx_xvhaddw_d_w(a: __v8i32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvhaddw.hu.bu"]
    fn __lasx_xvhaddw_hu_bu(a: __v32u8, b: __v32u8) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvhaddw.wu.hu"]
    fn __lasx_xvhaddw_wu_hu(a: __v16u16, b: __v16u16) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvhaddw.du.wu"]
    fn __lasx_xvhaddw_du_wu(a: __v8u32, b: __v8u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvhsubw.h.b"]
    fn __lasx_xvhsubw_h_b(a: __v32i8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvhsubw.w.h"]
    fn __lasx_xvhsubw_w_h(a: __v16i16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvhsubw.d.w"]
    fn __lasx_xvhsubw_d_w(a: __v8i32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvhsubw.hu.bu"]
    fn __lasx_xvhsubw_hu_bu(a: __v32u8, b: __v32u8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvhsubw.wu.hu"]
    fn __lasx_xvhsubw_wu_hu(a: __v16u16, b: __v16u16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvhsubw.du.wu"]
    fn __lasx_xvhsubw_du_wu(a: __v8u32, b: __v8u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmod.b"]
    fn __lasx_xvmod_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvmod.h"]
    fn __lasx_xvmod_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmod.w"]
    fn __lasx_xvmod_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmod.d"]
    fn __lasx_xvmod_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmod.bu"]
    fn __lasx_xvmod_bu(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvmod.hu"]
    fn __lasx_xvmod_hu(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvmod.wu"]
    fn __lasx_xvmod_wu(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvmod.du"]
    fn __lasx_xvmod_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvrepl128vei.b"]
    fn __lasx_xvrepl128vei_b(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvrepl128vei.h"]
    fn __lasx_xvrepl128vei_h(a: __v16i16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvrepl128vei.w"]
    fn __lasx_xvrepl128vei_w(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvrepl128vei.d"]
    fn __lasx_xvrepl128vei_d(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvpickev.b"]
    fn __lasx_xvpickev_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvpickev.h"]
    fn __lasx_xvpickev_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvpickev.w"]
    fn __lasx_xvpickev_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvpickev.d"]
    fn __lasx_xvpickev_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvpickod.b"]
    fn __lasx_xvpickod_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvpickod.h"]
    fn __lasx_xvpickod_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvpickod.w"]
    fn __lasx_xvpickod_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvpickod.d"]
    fn __lasx_xvpickod_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvilvh.b"]
    fn __lasx_xvilvh_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvilvh.h"]
    fn __lasx_xvilvh_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvilvh.w"]
    fn __lasx_xvilvh_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvilvh.d"]
    fn __lasx_xvilvh_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvilvl.b"]
    fn __lasx_xvilvl_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvilvl.h"]
    fn __lasx_xvilvl_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvilvl.w"]
    fn __lasx_xvilvl_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvilvl.d"]
    fn __lasx_xvilvl_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvpackev.b"]
    fn __lasx_xvpackev_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvpackev.h"]
    fn __lasx_xvpackev_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvpackev.w"]
    fn __lasx_xvpackev_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvpackev.d"]
    fn __lasx_xvpackev_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvpackod.b"]
    fn __lasx_xvpackod_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvpackod.h"]
    fn __lasx_xvpackod_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvpackod.w"]
    fn __lasx_xvpackod_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvpackod.d"]
    fn __lasx_xvpackod_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvshuf.b"]
    fn __lasx_xvshuf_b(a: __v32i8, b: __v32i8, c: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvshuf.h"]
    fn __lasx_xvshuf_h(a: __v16i16, b: __v16i16, c: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvshuf.w"]
    fn __lasx_xvshuf_w(a: __v8i32, b: __v8i32, c: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvshuf.d"]
    fn __lasx_xvshuf_d(a: __v4i64, b: __v4i64, c: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvand.v"]
    fn __lasx_xvand_v(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvandi.b"]
    fn __lasx_xvandi_b(a: __v32u8, b: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvor.v"]
    fn __lasx_xvor_v(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvori.b"]
    fn __lasx_xvori_b(a: __v32u8, b: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvnor.v"]
    fn __lasx_xvnor_v(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvnori.b"]
    fn __lasx_xvnori_b(a: __v32u8, b: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvxor.v"]
    fn __lasx_xvxor_v(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvxori.b"]
    fn __lasx_xvxori_b(a: __v32u8, b: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvbitsel.v"]
    fn __lasx_xvbitsel_v(a: __v32u8, b: __v32u8, c: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvbitseli.b"]
    fn __lasx_xvbitseli_b(a: __v32u8, b: __v32u8, c: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvshuf4i.b"]
    fn __lasx_xvshuf4i_b(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvshuf4i.h"]
    fn __lasx_xvshuf4i_h(a: __v16i16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvshuf4i.w"]
    fn __lasx_xvshuf4i_w(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvreplgr2vr.b"]
    fn __lasx_xvreplgr2vr_b(a: i32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvreplgr2vr.h"]
    fn __lasx_xvreplgr2vr_h(a: i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvreplgr2vr.w"]
    fn __lasx_xvreplgr2vr_w(a: i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvreplgr2vr.d"]
    fn __lasx_xvreplgr2vr_d(a: i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvpcnt.b"]
    fn __lasx_xvpcnt_b(a: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvpcnt.h"]
    fn __lasx_xvpcnt_h(a: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvpcnt.w"]
    fn __lasx_xvpcnt_w(a: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvpcnt.d"]
    fn __lasx_xvpcnt_d(a: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvclo.b"]
    fn __lasx_xvclo_b(a: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvclo.h"]
    fn __lasx_xvclo_h(a: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvclo.w"]
    fn __lasx_xvclo_w(a: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvclo.d"]
    fn __lasx_xvclo_d(a: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvclz.b"]
    fn __lasx_xvclz_b(a: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvclz.h"]
    fn __lasx_xvclz_h(a: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvclz.w"]
    fn __lasx_xvclz_w(a: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvclz.d"]
    fn __lasx_xvclz_d(a: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfadd.s"]
    fn __lasx_xvfadd_s(a: __v8f32, b: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfadd.d"]
    fn __lasx_xvfadd_d(a: __v4f64, b: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfsub.s"]
    fn __lasx_xvfsub_s(a: __v8f32, b: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfsub.d"]
    fn __lasx_xvfsub_d(a: __v4f64, b: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfmul.s"]
    fn __lasx_xvfmul_s(a: __v8f32, b: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfmul.d"]
    fn __lasx_xvfmul_d(a: __v4f64, b: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfdiv.s"]
    fn __lasx_xvfdiv_s(a: __v8f32, b: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfdiv.d"]
    fn __lasx_xvfdiv_d(a: __v4f64, b: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfcvt.h.s"]
    fn __lasx_xvfcvt_h_s(a: __v8f32, b: __v8f32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvfcvt.s.d"]
    fn __lasx_xvfcvt_s_d(a: __v4f64, b: __v4f64) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfmin.s"]
    fn __lasx_xvfmin_s(a: __v8f32, b: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfmin.d"]
    fn __lasx_xvfmin_d(a: __v4f64, b: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfmina.s"]
    fn __lasx_xvfmina_s(a: __v8f32, b: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfmina.d"]
    fn __lasx_xvfmina_d(a: __v4f64, b: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfmax.s"]
    fn __lasx_xvfmax_s(a: __v8f32, b: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfmax.d"]
    fn __lasx_xvfmax_d(a: __v4f64, b: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfmaxa.s"]
    fn __lasx_xvfmaxa_s(a: __v8f32, b: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfmaxa.d"]
    fn __lasx_xvfmaxa_d(a: __v4f64, b: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfclass.s"]
    fn __lasx_xvfclass_s(a: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfclass.d"]
    fn __lasx_xvfclass_d(a: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfsqrt.s"]
    fn __lasx_xvfsqrt_s(a: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfsqrt.d"]
    fn __lasx_xvfsqrt_d(a: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfrecip.s"]
    fn __lasx_xvfrecip_s(a: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfrecip.d"]
    fn __lasx_xvfrecip_d(a: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfrecipe.s"]
    fn __lasx_xvfrecipe_s(a: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfrecipe.d"]
    fn __lasx_xvfrecipe_d(a: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfrsqrte.s"]
    fn __lasx_xvfrsqrte_s(a: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfrsqrte.d"]
    fn __lasx_xvfrsqrte_d(a: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfrint.s"]
    fn __lasx_xvfrint_s(a: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfrint.d"]
    fn __lasx_xvfrint_d(a: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfrsqrt.s"]
    fn __lasx_xvfrsqrt_s(a: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfrsqrt.d"]
    fn __lasx_xvfrsqrt_d(a: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvflogb.s"]
    fn __lasx_xvflogb_s(a: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvflogb.d"]
    fn __lasx_xvflogb_d(a: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfcvth.s.h"]
    fn __lasx_xvfcvth_s_h(a: __v16i16) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfcvth.d.s"]
    fn __lasx_xvfcvth_d_s(a: __v8f32) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfcvtl.s.h"]
    fn __lasx_xvfcvtl_s_h(a: __v16i16) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfcvtl.d.s"]
    fn __lasx_xvfcvtl_d_s(a: __v8f32) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvftint.w.s"]
    fn __lasx_xvftint_w_s(a: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvftint.l.d"]
    fn __lasx_xvftint_l_d(a: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftint.wu.s"]
    fn __lasx_xvftint_wu_s(a: __v8f32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvftint.lu.d"]
    fn __lasx_xvftint_lu_d(a: __v4f64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvftintrz.w.s"]
    fn __lasx_xvftintrz_w_s(a: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvftintrz.l.d"]
    fn __lasx_xvftintrz_l_d(a: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftintrz.wu.s"]
    fn __lasx_xvftintrz_wu_s(a: __v8f32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvftintrz.lu.d"]
    fn __lasx_xvftintrz_lu_d(a: __v4f64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvffint.s.w"]
    fn __lasx_xvffint_s_w(a: __v8i32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvffint.d.l"]
    fn __lasx_xvffint_d_l(a: __v4i64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvffint.s.wu"]
    fn __lasx_xvffint_s_wu(a: __v8u32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvffint.d.lu"]
    fn __lasx_xvffint_d_lu(a: __v4u64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvreplve.b"]
    fn __lasx_xvreplve_b(a: __v32i8, b: i32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvreplve.h"]
    fn __lasx_xvreplve_h(a: __v16i16, b: i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvreplve.w"]
    fn __lasx_xvreplve_w(a: __v8i32, b: i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvreplve.d"]
    fn __lasx_xvreplve_d(a: __v4i64, b: i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvpermi.w"]
    fn __lasx_xvpermi_w(a: __v8i32, b: __v8i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvandn.v"]
    fn __lasx_xvandn_v(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvneg.b"]
    fn __lasx_xvneg_b(a: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvneg.h"]
    fn __lasx_xvneg_h(a: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvneg.w"]
    fn __lasx_xvneg_w(a: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvneg.d"]
    fn __lasx_xvneg_d(a: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmuh.b"]
    fn __lasx_xvmuh_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvmuh.h"]
    fn __lasx_xvmuh_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmuh.w"]
    fn __lasx_xvmuh_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmuh.d"]
    fn __lasx_xvmuh_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmuh.bu"]
    fn __lasx_xvmuh_bu(a: __v32u8, b: __v32u8) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvmuh.hu"]
    fn __lasx_xvmuh_hu(a: __v16u16, b: __v16u16) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvmuh.wu"]
    fn __lasx_xvmuh_wu(a: __v8u32, b: __v8u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvmuh.du"]
    fn __lasx_xvmuh_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvsllwil.h.b"]
    fn __lasx_xvsllwil_h_b(a: __v32i8, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsllwil.w.h"]
    fn __lasx_xvsllwil_w_h(a: __v16i16, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsllwil.d.w"]
    fn __lasx_xvsllwil_d_w(a: __v8i32, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsllwil.hu.bu"]
    fn __lasx_xvsllwil_hu_bu(a: __v32u8, b: u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvsllwil.wu.hu"]
    fn __lasx_xvsllwil_wu_hu(a: __v16u16, b: u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvsllwil.du.wu"]
    fn __lasx_xvsllwil_du_wu(a: __v8u32, b: u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvsran.b.h"]
    fn __lasx_xvsran_b_h(a: __v16i16, b: __v16i16) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsran.h.w"]
    fn __lasx_xvsran_h_w(a: __v8i32, b: __v8i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsran.w.d"]
    fn __lasx_xvsran_w_d(a: __v4i64, b: __v4i64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssran.b.h"]
    fn __lasx_xvssran_b_h(a: __v16i16, b: __v16i16) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvssran.h.w"]
    fn __lasx_xvssran_h_w(a: __v8i32, b: __v8i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvssran.w.d"]
    fn __lasx_xvssran_w_d(a: __v4i64, b: __v4i64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssran.bu.h"]
    fn __lasx_xvssran_bu_h(a: __v16u16, b: __v16u16) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvssran.hu.w"]
    fn __lasx_xvssran_hu_w(a: __v8u32, b: __v8u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvssran.wu.d"]
    fn __lasx_xvssran_wu_d(a: __v4u64, b: __v4u64) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvsrarn.b.h"]
    fn __lasx_xvsrarn_b_h(a: __v16i16, b: __v16i16) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrarn.h.w"]
    fn __lasx_xvsrarn_h_w(a: __v8i32, b: __v8i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrarn.w.d"]
    fn __lasx_xvsrarn_w_d(a: __v4i64, b: __v4i64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssrarn.b.h"]
    fn __lasx_xvssrarn_b_h(a: __v16i16, b: __v16i16) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvssrarn.h.w"]
    fn __lasx_xvssrarn_h_w(a: __v8i32, b: __v8i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvssrarn.w.d"]
    fn __lasx_xvssrarn_w_d(a: __v4i64, b: __v4i64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssrarn.bu.h"]
    fn __lasx_xvssrarn_bu_h(a: __v16u16, b: __v16u16) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvssrarn.hu.w"]
    fn __lasx_xvssrarn_hu_w(a: __v8u32, b: __v8u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvssrarn.wu.d"]
    fn __lasx_xvssrarn_wu_d(a: __v4u64, b: __v4u64) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvsrln.b.h"]
    fn __lasx_xvsrln_b_h(a: __v16i16, b: __v16i16) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrln.h.w"]
    fn __lasx_xvsrln_h_w(a: __v8i32, b: __v8i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrln.w.d"]
    fn __lasx_xvsrln_w_d(a: __v4i64, b: __v4i64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssrln.bu.h"]
    fn __lasx_xvssrln_bu_h(a: __v16u16, b: __v16u16) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvssrln.hu.w"]
    fn __lasx_xvssrln_hu_w(a: __v8u32, b: __v8u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvssrln.wu.d"]
    fn __lasx_xvssrln_wu_d(a: __v4u64, b: __v4u64) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvsrlrn.b.h"]
    fn __lasx_xvsrlrn_b_h(a: __v16i16, b: __v16i16) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrlrn.h.w"]
    fn __lasx_xvsrlrn_h_w(a: __v8i32, b: __v8i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrlrn.w.d"]
    fn __lasx_xvsrlrn_w_d(a: __v4i64, b: __v4i64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssrlrn.bu.h"]
    fn __lasx_xvssrlrn_bu_h(a: __v16u16, b: __v16u16) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvssrlrn.hu.w"]
    fn __lasx_xvssrlrn_hu_w(a: __v8u32, b: __v8u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvssrlrn.wu.d"]
    fn __lasx_xvssrlrn_wu_d(a: __v4u64, b: __v4u64) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvfrstpi.b"]
    fn __lasx_xvfrstpi_b(a: __v32i8, b: __v32i8, c: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvfrstpi.h"]
    fn __lasx_xvfrstpi_h(a: __v16i16, b: __v16i16, c: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvfrstp.b"]
    fn __lasx_xvfrstp_b(a: __v32i8, b: __v32i8, c: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvfrstp.h"]
    fn __lasx_xvfrstp_h(a: __v16i16, b: __v16i16, c: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvshuf4i.d"]
    fn __lasx_xvshuf4i_d(a: __v4i64, b: __v4i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvbsrl.v"]
    fn __lasx_xvbsrl_v(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvbsll.v"]
    fn __lasx_xvbsll_v(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvextrins.b"]
    fn __lasx_xvextrins_b(a: __v32i8, b: __v32i8, c: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvextrins.h"]
    fn __lasx_xvextrins_h(a: __v16i16, b: __v16i16, c: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvextrins.w"]
    fn __lasx_xvextrins_w(a: __v8i32, b: __v8i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvextrins.d"]
    fn __lasx_xvextrins_d(a: __v4i64, b: __v4i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmskltz.b"]
    fn __lasx_xvmskltz_b(a: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvmskltz.h"]
    fn __lasx_xvmskltz_h(a: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmskltz.w"]
    fn __lasx_xvmskltz_w(a: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmskltz.d"]
    fn __lasx_xvmskltz_d(a: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsigncov.b"]
    fn __lasx_xvsigncov_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsigncov.h"]
    fn __lasx_xvsigncov_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsigncov.w"]
    fn __lasx_xvsigncov_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsigncov.d"]
    fn __lasx_xvsigncov_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfmadd.s"]
    fn __lasx_xvfmadd_s(a: __v8f32, b: __v8f32, c: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfmadd.d"]
    fn __lasx_xvfmadd_d(a: __v4f64, b: __v4f64, c: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfmsub.s"]
    fn __lasx_xvfmsub_s(a: __v8f32, b: __v8f32, c: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfmsub.d"]
    fn __lasx_xvfmsub_d(a: __v4f64, b: __v4f64, c: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfnmadd.s"]
    fn __lasx_xvfnmadd_s(a: __v8f32, b: __v8f32, c: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfnmadd.d"]
    fn __lasx_xvfnmadd_d(a: __v4f64, b: __v4f64, c: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfnmsub.s"]
    fn __lasx_xvfnmsub_s(a: __v8f32, b: __v8f32, c: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfnmsub.d"]
    fn __lasx_xvfnmsub_d(a: __v4f64, b: __v4f64, c: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvftintrne.w.s"]
    fn __lasx_xvftintrne_w_s(a: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvftintrne.l.d"]
    fn __lasx_xvftintrne_l_d(a: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftintrp.w.s"]
    fn __lasx_xvftintrp_w_s(a: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvftintrp.l.d"]
    fn __lasx_xvftintrp_l_d(a: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftintrm.w.s"]
    fn __lasx_xvftintrm_w_s(a: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvftintrm.l.d"]
    fn __lasx_xvftintrm_l_d(a: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftint.w.d"]
    fn __lasx_xvftint_w_d(a: __v4f64, b: __v4f64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvffint.s.l"]
    fn __lasx_xvffint_s_l(a: __v4i64, b: __v4i64) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvftintrz.w.d"]
    fn __lasx_xvftintrz_w_d(a: __v4f64, b: __v4f64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvftintrp.w.d"]
    fn __lasx_xvftintrp_w_d(a: __v4f64, b: __v4f64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvftintrm.w.d"]
    fn __lasx_xvftintrm_w_d(a: __v4f64, b: __v4f64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvftintrne.w.d"]
    fn __lasx_xvftintrne_w_d(a: __v4f64, b: __v4f64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvftinth.l.s"]
    fn __lasx_xvftinth_l_s(a: __v8f32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftintl.l.s"]
    fn __lasx_xvftintl_l_s(a: __v8f32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvffinth.d.w"]
    fn __lasx_xvffinth_d_w(a: __v8i32) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvffintl.d.w"]
    fn __lasx_xvffintl_d_w(a: __v8i32) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvftintrzh.l.s"]
    fn __lasx_xvftintrzh_l_s(a: __v8f32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftintrzl.l.s"]
    fn __lasx_xvftintrzl_l_s(a: __v8f32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftintrph.l.s"]
    fn __lasx_xvftintrph_l_s(a: __v8f32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftintrpl.l.s"]
    fn __lasx_xvftintrpl_l_s(a: __v8f32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftintrmh.l.s"]
    fn __lasx_xvftintrmh_l_s(a: __v8f32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftintrml.l.s"]
    fn __lasx_xvftintrml_l_s(a: __v8f32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftintrneh.l.s"]
    fn __lasx_xvftintrneh_l_s(a: __v8f32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvftintrnel.l.s"]
    fn __lasx_xvftintrnel_l_s(a: __v8f32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfrintrne.s"]
    fn __lasx_xvfrintrne_s(a: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfrintrne.d"]
    fn __lasx_xvfrintrne_d(a: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfrintrz.s"]
    fn __lasx_xvfrintrz_s(a: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfrintrz.d"]
    fn __lasx_xvfrintrz_d(a: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfrintrp.s"]
    fn __lasx_xvfrintrp_s(a: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfrintrp.d"]
    fn __lasx_xvfrintrp_d(a: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvfrintrm.s"]
    fn __lasx_xvfrintrm_s(a: __v8f32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvfrintrm.d"]
    fn __lasx_xvfrintrm_d(a: __v4f64) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvld"]
    fn __lasx_xvld(a: *const i8, b: i32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvst"]
    fn __lasx_xvst(a: __v32i8, b: *mut i8, c: i32);
    #[link_name = "llvm.loongarch.lasx.xvstelm.b"]
    fn __lasx_xvstelm_b(a: __v32i8, b: *mut i8, c: i32, d: u32);
    #[link_name = "llvm.loongarch.lasx.xvstelm.h"]
    fn __lasx_xvstelm_h(a: __v16i16, b: *mut i8, c: i32, d: u32);
    #[link_name = "llvm.loongarch.lasx.xvstelm.w"]
    fn __lasx_xvstelm_w(a: __v8i32, b: *mut i8, c: i32, d: u32);
    #[link_name = "llvm.loongarch.lasx.xvstelm.d"]
    fn __lasx_xvstelm_d(a: __v4i64, b: *mut i8, c: i32, d: u32);
    #[link_name = "llvm.loongarch.lasx.xvinsve0.w"]
    fn __lasx_xvinsve0_w(a: __v8i32, b: __v8i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvinsve0.d"]
    fn __lasx_xvinsve0_d(a: __v4i64, b: __v4i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvpickve.w"]
    fn __lasx_xvpickve_w(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvpickve.d"]
    fn __lasx_xvpickve_d(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvssrlrn.b.h"]
    fn __lasx_xvssrlrn_b_h(a: __v16i16, b: __v16i16) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvssrlrn.h.w"]
    fn __lasx_xvssrlrn_h_w(a: __v8i32, b: __v8i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvssrlrn.w.d"]
    fn __lasx_xvssrlrn_w_d(a: __v4i64, b: __v4i64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssrln.b.h"]
    fn __lasx_xvssrln_b_h(a: __v16i16, b: __v16i16) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvssrln.h.w"]
    fn __lasx_xvssrln_h_w(a: __v8i32, b: __v8i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvssrln.w.d"]
    fn __lasx_xvssrln_w_d(a: __v4i64, b: __v4i64) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvorn.v"]
    fn __lasx_xvorn_v(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvldi"]
    fn __lasx_xvldi(a: i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvldx"]
    fn __lasx_xvldx(a: *const i8, b: i64) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvstx"]
    fn __lasx_xvstx(a: __v32i8, b: *mut i8, c: i64);
    #[link_name = "llvm.loongarch.lasx.xvextl.qu.du"]
    fn __lasx_xvextl_qu_du(a: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvinsgr2vr.w"]
    fn __lasx_xvinsgr2vr_w(a: __v8i32, b: i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvinsgr2vr.d"]
    fn __lasx_xvinsgr2vr_d(a: __v4i64, b: i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvreplve0.b"]
    fn __lasx_xvreplve0_b(a: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvreplve0.h"]
    fn __lasx_xvreplve0_h(a: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvreplve0.w"]
    fn __lasx_xvreplve0_w(a: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvreplve0.d"]
    fn __lasx_xvreplve0_d(a: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvreplve0.q"]
    fn __lasx_xvreplve0_q(a: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.vext2xv.h.b"]
    fn __lasx_vext2xv_h_b(a: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.vext2xv.w.h"]
    fn __lasx_vext2xv_w_h(a: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.vext2xv.d.w"]
    fn __lasx_vext2xv_d_w(a: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.vext2xv.w.b"]
    fn __lasx_vext2xv_w_b(a: __v32i8) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.vext2xv.d.h"]
    fn __lasx_vext2xv_d_h(a: __v16i16) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.vext2xv.d.b"]
    fn __lasx_vext2xv_d_b(a: __v32i8) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.vext2xv.hu.bu"]
    fn __lasx_vext2xv_hu_bu(a: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.vext2xv.wu.hu"]
    fn __lasx_vext2xv_wu_hu(a: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.vext2xv.du.wu"]
    fn __lasx_vext2xv_du_wu(a: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.vext2xv.wu.bu"]
    fn __lasx_vext2xv_wu_bu(a: __v32i8) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.vext2xv.du.hu"]
    fn __lasx_vext2xv_du_hu(a: __v16i16) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.vext2xv.du.bu"]
    fn __lasx_vext2xv_du_bu(a: __v32i8) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvpermi.q"]
    fn __lasx_xvpermi_q(a: __v32i8, b: __v32i8, c: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvpermi.d"]
    fn __lasx_xvpermi_d(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvperm.w"]
    fn __lasx_xvperm_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvldrepl.b"]
    fn __lasx_xvldrepl_b(a: *const i8, b: i32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvldrepl.h"]
    fn __lasx_xvldrepl_h(a: *const i8, b: i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvldrepl.w"]
    fn __lasx_xvldrepl_w(a: *const i8, b: i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvldrepl.d"]
    fn __lasx_xvldrepl_d(a: *const i8, b: i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvpickve2gr.w"]
    fn __lasx_xvpickve2gr_w(a: __v8i32, b: u32) -> i32;
    #[link_name = "llvm.loongarch.lasx.xvpickve2gr.wu"]
    fn __lasx_xvpickve2gr_wu(a: __v8i32, b: u32) -> u32;
    #[link_name = "llvm.loongarch.lasx.xvpickve2gr.d"]
    fn __lasx_xvpickve2gr_d(a: __v4i64, b: u32) -> i64;
    #[link_name = "llvm.loongarch.lasx.xvpickve2gr.du"]
    fn __lasx_xvpickve2gr_du(a: __v4i64, b: u32) -> u64;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.q.d"]
    fn __lasx_xvaddwev_q_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.d.w"]
    fn __lasx_xvaddwev_d_w(a: __v8i32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.w.h"]
    fn __lasx_xvaddwev_w_h(a: __v16i16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.h.b"]
    fn __lasx_xvaddwev_h_b(a: __v32i8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.q.du"]
    fn __lasx_xvaddwev_q_du(a: __v4u64, b: __v4u64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.d.wu"]
    fn __lasx_xvaddwev_d_wu(a: __v8u32, b: __v8u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.w.hu"]
    fn __lasx_xvaddwev_w_hu(a: __v16u16, b: __v16u16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.h.bu"]
    fn __lasx_xvaddwev_h_bu(a: __v32u8, b: __v32u8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsubwev.q.d"]
    fn __lasx_xvsubwev_q_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsubwev.d.w"]
    fn __lasx_xvsubwev_d_w(a: __v8i32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsubwev.w.h"]
    fn __lasx_xvsubwev_w_h(a: __v16i16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsubwev.h.b"]
    fn __lasx_xvsubwev_h_b(a: __v32i8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsubwev.q.du"]
    fn __lasx_xvsubwev_q_du(a: __v4u64, b: __v4u64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsubwev.d.wu"]
    fn __lasx_xvsubwev_d_wu(a: __v8u32, b: __v8u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsubwev.w.hu"]
    fn __lasx_xvsubwev_w_hu(a: __v16u16, b: __v16u16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsubwev.h.bu"]
    fn __lasx_xvsubwev_h_bu(a: __v32u8, b: __v32u8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.q.d"]
    fn __lasx_xvmulwev_q_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.d.w"]
    fn __lasx_xvmulwev_d_w(a: __v8i32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.w.h"]
    fn __lasx_xvmulwev_w_h(a: __v16i16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.h.b"]
    fn __lasx_xvmulwev_h_b(a: __v32i8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.q.du"]
    fn __lasx_xvmulwev_q_du(a: __v4u64, b: __v4u64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.d.wu"]
    fn __lasx_xvmulwev_d_wu(a: __v8u32, b: __v8u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.w.hu"]
    fn __lasx_xvmulwev_w_hu(a: __v16u16, b: __v16u16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.h.bu"]
    fn __lasx_xvmulwev_h_bu(a: __v32u8, b: __v32u8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.q.d"]
    fn __lasx_xvaddwod_q_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.d.w"]
    fn __lasx_xvaddwod_d_w(a: __v8i32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.w.h"]
    fn __lasx_xvaddwod_w_h(a: __v16i16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.h.b"]
    fn __lasx_xvaddwod_h_b(a: __v32i8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.q.du"]
    fn __lasx_xvaddwod_q_du(a: __v4u64, b: __v4u64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.d.wu"]
    fn __lasx_xvaddwod_d_wu(a: __v8u32, b: __v8u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.w.hu"]
    fn __lasx_xvaddwod_w_hu(a: __v16u16, b: __v16u16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.h.bu"]
    fn __lasx_xvaddwod_h_bu(a: __v32u8, b: __v32u8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsubwod.q.d"]
    fn __lasx_xvsubwod_q_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsubwod.d.w"]
    fn __lasx_xvsubwod_d_w(a: __v8i32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsubwod.w.h"]
    fn __lasx_xvsubwod_w_h(a: __v16i16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsubwod.h.b"]
    fn __lasx_xvsubwod_h_b(a: __v32i8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsubwod.q.du"]
    fn __lasx_xvsubwod_q_du(a: __v4u64, b: __v4u64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsubwod.d.wu"]
    fn __lasx_xvsubwod_d_wu(a: __v8u32, b: __v8u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsubwod.w.hu"]
    fn __lasx_xvsubwod_w_hu(a: __v16u16, b: __v16u16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsubwod.h.bu"]
    fn __lasx_xvsubwod_h_bu(a: __v32u8, b: __v32u8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.q.d"]
    fn __lasx_xvmulwod_q_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.d.w"]
    fn __lasx_xvmulwod_d_w(a: __v8i32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.w.h"]
    fn __lasx_xvmulwod_w_h(a: __v16i16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.h.b"]
    fn __lasx_xvmulwod_h_b(a: __v32i8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.q.du"]
    fn __lasx_xvmulwod_q_du(a: __v4u64, b: __v4u64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.d.wu"]
    fn __lasx_xvmulwod_d_wu(a: __v8u32, b: __v8u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.w.hu"]
    fn __lasx_xvmulwod_w_hu(a: __v16u16, b: __v16u16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.h.bu"]
    fn __lasx_xvmulwod_h_bu(a: __v32u8, b: __v32u8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.d.wu.w"]
    fn __lasx_xvaddwev_d_wu_w(a: __v8u32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.w.hu.h"]
    fn __lasx_xvaddwev_w_hu_h(a: __v16u16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.h.bu.b"]
    fn __lasx_xvaddwev_h_bu_b(a: __v32u8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.d.wu.w"]
    fn __lasx_xvmulwev_d_wu_w(a: __v8u32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.w.hu.h"]
    fn __lasx_xvmulwev_w_hu_h(a: __v16u16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.h.bu.b"]
    fn __lasx_xvmulwev_h_bu_b(a: __v32u8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.d.wu.w"]
    fn __lasx_xvaddwod_d_wu_w(a: __v8u32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.w.hu.h"]
    fn __lasx_xvaddwod_w_hu_h(a: __v16u16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.h.bu.b"]
    fn __lasx_xvaddwod_h_bu_b(a: __v32u8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.d.wu.w"]
    fn __lasx_xvmulwod_d_wu_w(a: __v8u32, b: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.w.hu.h"]
    fn __lasx_xvmulwod_w_hu_h(a: __v16u16, b: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.h.bu.b"]
    fn __lasx_xvmulwod_h_bu_b(a: __v32u8, b: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvhaddw.q.d"]
    fn __lasx_xvhaddw_q_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvhaddw.qu.du"]
    fn __lasx_xvhaddw_qu_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvhsubw.q.d"]
    fn __lasx_xvhsubw_q_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvhsubw.qu.du"]
    fn __lasx_xvhsubw_qu_du(a: __v4u64, b: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.q.d"]
    fn __lasx_xvmaddwev_q_d(a: __v4i64, b: __v4i64, c: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.d.w"]
    fn __lasx_xvmaddwev_d_w(a: __v4i64, b: __v8i32, c: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.w.h"]
    fn __lasx_xvmaddwev_w_h(a: __v8i32, b: __v16i16, c: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.h.b"]
    fn __lasx_xvmaddwev_h_b(a: __v16i16, b: __v32i8, c: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.q.du"]
    fn __lasx_xvmaddwev_q_du(a: __v4u64, b: __v4u64, c: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.d.wu"]
    fn __lasx_xvmaddwev_d_wu(a: __v4u64, b: __v8u32, c: __v8u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.w.hu"]
    fn __lasx_xvmaddwev_w_hu(a: __v8u32, b: __v16u16, c: __v16u16) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.h.bu"]
    fn __lasx_xvmaddwev_h_bu(a: __v16u16, b: __v32u8, c: __v32u8) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.q.d"]
    fn __lasx_xvmaddwod_q_d(a: __v4i64, b: __v4i64, c: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.d.w"]
    fn __lasx_xvmaddwod_d_w(a: __v4i64, b: __v8i32, c: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.w.h"]
    fn __lasx_xvmaddwod_w_h(a: __v8i32, b: __v16i16, c: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.h.b"]
    fn __lasx_xvmaddwod_h_b(a: __v16i16, b: __v32i8, c: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.q.du"]
    fn __lasx_xvmaddwod_q_du(a: __v4u64, b: __v4u64, c: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.d.wu"]
    fn __lasx_xvmaddwod_d_wu(a: __v4u64, b: __v8u32, c: __v8u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.w.hu"]
    fn __lasx_xvmaddwod_w_hu(a: __v8u32, b: __v16u16, c: __v16u16) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.h.bu"]
    fn __lasx_xvmaddwod_h_bu(a: __v16u16, b: __v32u8, c: __v32u8) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.q.du.d"]
    fn __lasx_xvmaddwev_q_du_d(a: __v4i64, b: __v4u64, c: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.d.wu.w"]
    fn __lasx_xvmaddwev_d_wu_w(a: __v4i64, b: __v8u32, c: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.w.hu.h"]
    fn __lasx_xvmaddwev_w_hu_h(a: __v8i32, b: __v16u16, c: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmaddwev.h.bu.b"]
    fn __lasx_xvmaddwev_h_bu_b(a: __v16i16, b: __v32u8, c: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.q.du.d"]
    fn __lasx_xvmaddwod_q_du_d(a: __v4i64, b: __v4u64, c: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.d.wu.w"]
    fn __lasx_xvmaddwod_d_wu_w(a: __v4i64, b: __v8u32, c: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.w.hu.h"]
    fn __lasx_xvmaddwod_w_hu_h(a: __v8i32, b: __v16u16, c: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvmaddwod.h.bu.b"]
    fn __lasx_xvmaddwod_h_bu_b(a: __v16i16, b: __v32u8, c: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvrotr.b"]
    fn __lasx_xvrotr_b(a: __v32i8, b: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvrotr.h"]
    fn __lasx_xvrotr_h(a: __v16i16, b: __v16i16) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvrotr.w"]
    fn __lasx_xvrotr_w(a: __v8i32, b: __v8i32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvrotr.d"]
    fn __lasx_xvrotr_d(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvadd.q"]
    fn __lasx_xvadd_q(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsub.q"]
    fn __lasx_xvsub_q(a: __v4i64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwev.q.du.d"]
    fn __lasx_xvaddwev_q_du_d(a: __v4u64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvaddwod.q.du.d"]
    fn __lasx_xvaddwod_q_du_d(a: __v4u64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwev.q.du.d"]
    fn __lasx_xvmulwev_q_du_d(a: __v4u64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmulwod.q.du.d"]
    fn __lasx_xvmulwod_q_du_d(a: __v4u64, b: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvmskgez.b"]
    fn __lasx_xvmskgez_b(a: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvmsknz.b"]
    fn __lasx_xvmsknz_b(a: __v32i8) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvexth.h.b"]
    fn __lasx_xvexth_h_b(a: __v32i8) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvexth.w.h"]
    fn __lasx_xvexth_w_h(a: __v16i16) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvexth.d.w"]
    fn __lasx_xvexth_d_w(a: __v8i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvexth.q.d"]
    fn __lasx_xvexth_q_d(a: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvexth.hu.bu"]
    fn __lasx_xvexth_hu_bu(a: __v32u8) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvexth.wu.hu"]
    fn __lasx_xvexth_wu_hu(a: __v16u16) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvexth.du.wu"]
    fn __lasx_xvexth_du_wu(a: __v8u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvexth.qu.du"]
    fn __lasx_xvexth_qu_du(a: __v4u64) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvrotri.b"]
    fn __lasx_xvrotri_b(a: __v32i8, b: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvrotri.h"]
    fn __lasx_xvrotri_h(a: __v16i16, b: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvrotri.w"]
    fn __lasx_xvrotri_w(a: __v8i32, b: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvrotri.d"]
    fn __lasx_xvrotri_d(a: __v4i64, b: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvextl.q.d"]
    fn __lasx_xvextl_q_d(a: __v4i64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsrlni.b.h"]
    fn __lasx_xvsrlni_b_h(a: __v32i8, b: __v32i8, c: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrlni.h.w"]
    fn __lasx_xvsrlni_h_w(a: __v16i16, b: __v16i16, c: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrlni.w.d"]
    fn __lasx_xvsrlni_w_d(a: __v8i32, b: __v8i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsrlni.d.q"]
    fn __lasx_xvsrlni_d_q(a: __v4i64, b: __v4i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsrlrni.b.h"]
    fn __lasx_xvsrlrni_b_h(a: __v32i8, b: __v32i8, c: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrlrni.h.w"]
    fn __lasx_xvsrlrni_h_w(a: __v16i16, b: __v16i16, c: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrlrni.w.d"]
    fn __lasx_xvsrlrni_w_d(a: __v8i32, b: __v8i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsrlrni.d.q"]
    fn __lasx_xvsrlrni_d_q(a: __v4i64, b: __v4i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvssrlni.b.h"]
    fn __lasx_xvssrlni_b_h(a: __v32i8, b: __v32i8, c: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvssrlni.h.w"]
    fn __lasx_xvssrlni_h_w(a: __v16i16, b: __v16i16, c: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvssrlni.w.d"]
    fn __lasx_xvssrlni_w_d(a: __v8i32, b: __v8i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssrlni.d.q"]
    fn __lasx_xvssrlni_d_q(a: __v4i64, b: __v4i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvssrlni.bu.h"]
    fn __lasx_xvssrlni_bu_h(a: __v32u8, b: __v32i8, c: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvssrlni.hu.w"]
    fn __lasx_xvssrlni_hu_w(a: __v16u16, b: __v16i16, c: u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvssrlni.wu.d"]
    fn __lasx_xvssrlni_wu_d(a: __v8u32, b: __v8i32, c: u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvssrlni.du.q"]
    fn __lasx_xvssrlni_du_q(a: __v4u64, b: __v4i64, c: u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvssrlrni.b.h"]
    fn __lasx_xvssrlrni_b_h(a: __v32i8, b: __v32i8, c: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvssrlrni.h.w"]
    fn __lasx_xvssrlrni_h_w(a: __v16i16, b: __v16i16, c: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvssrlrni.w.d"]
    fn __lasx_xvssrlrni_w_d(a: __v8i32, b: __v8i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssrlrni.d.q"]
    fn __lasx_xvssrlrni_d_q(a: __v4i64, b: __v4i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvssrlrni.bu.h"]
    fn __lasx_xvssrlrni_bu_h(a: __v32u8, b: __v32i8, c: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvssrlrni.hu.w"]
    fn __lasx_xvssrlrni_hu_w(a: __v16u16, b: __v16i16, c: u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvssrlrni.wu.d"]
    fn __lasx_xvssrlrni_wu_d(a: __v8u32, b: __v8i32, c: u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvssrlrni.du.q"]
    fn __lasx_xvssrlrni_du_q(a: __v4u64, b: __v4i64, c: u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvsrani.b.h"]
    fn __lasx_xvsrani_b_h(a: __v32i8, b: __v32i8, c: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrani.h.w"]
    fn __lasx_xvsrani_h_w(a: __v16i16, b: __v16i16, c: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrani.w.d"]
    fn __lasx_xvsrani_w_d(a: __v8i32, b: __v8i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsrani.d.q"]
    fn __lasx_xvsrani_d_q(a: __v4i64, b: __v4i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvsrarni.b.h"]
    fn __lasx_xvsrarni_b_h(a: __v32i8, b: __v32i8, c: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvsrarni.h.w"]
    fn __lasx_xvsrarni_h_w(a: __v16i16, b: __v16i16, c: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvsrarni.w.d"]
    fn __lasx_xvsrarni_w_d(a: __v8i32, b: __v8i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvsrarni.d.q"]
    fn __lasx_xvsrarni_d_q(a: __v4i64, b: __v4i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvssrani.b.h"]
    fn __lasx_xvssrani_b_h(a: __v32i8, b: __v32i8, c: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvssrani.h.w"]
    fn __lasx_xvssrani_h_w(a: __v16i16, b: __v16i16, c: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvssrani.w.d"]
    fn __lasx_xvssrani_w_d(a: __v8i32, b: __v8i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssrani.d.q"]
    fn __lasx_xvssrani_d_q(a: __v4i64, b: __v4i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvssrani.bu.h"]
    fn __lasx_xvssrani_bu_h(a: __v32u8, b: __v32i8, c: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvssrani.hu.w"]
    fn __lasx_xvssrani_hu_w(a: __v16u16, b: __v16i16, c: u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvssrani.wu.d"]
    fn __lasx_xvssrani_wu_d(a: __v8u32, b: __v8i32, c: u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvssrani.du.q"]
    fn __lasx_xvssrani_du_q(a: __v4u64, b: __v4i64, c: u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xvssrarni.b.h"]
    fn __lasx_xvssrarni_b_h(a: __v32i8, b: __v32i8, c: u32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvssrarni.h.w"]
    fn __lasx_xvssrarni_h_w(a: __v16i16, b: __v16i16, c: u32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvssrarni.w.d"]
    fn __lasx_xvssrarni_w_d(a: __v8i32, b: __v8i32, c: u32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvssrarni.d.q"]
    fn __lasx_xvssrarni_d_q(a: __v4i64, b: __v4i64, c: u32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvssrarni.bu.h"]
    fn __lasx_xvssrarni_bu_h(a: __v32u8, b: __v32i8, c: u32) -> __v32u8;
    #[link_name = "llvm.loongarch.lasx.xvssrarni.hu.w"]
    fn __lasx_xvssrarni_hu_w(a: __v16u16, b: __v16i16, c: u32) -> __v16u16;
    #[link_name = "llvm.loongarch.lasx.xvssrarni.wu.d"]
    fn __lasx_xvssrarni_wu_d(a: __v8u32, b: __v8i32, c: u32) -> __v8u32;
    #[link_name = "llvm.loongarch.lasx.xvssrarni.du.q"]
    fn __lasx_xvssrarni_du_q(a: __v4u64, b: __v4i64, c: u32) -> __v4u64;
    #[link_name = "llvm.loongarch.lasx.xbnz.b"]
    fn __lasx_xbnz_b(a: __v32u8) -> i32;
    #[link_name = "llvm.loongarch.lasx.xbnz.d"]
    fn __lasx_xbnz_d(a: __v4u64) -> i32;
    #[link_name = "llvm.loongarch.lasx.xbnz.h"]
    fn __lasx_xbnz_h(a: __v16u16) -> i32;
    #[link_name = "llvm.loongarch.lasx.xbnz.v"]
    fn __lasx_xbnz_v(a: __v32u8) -> i32;
    #[link_name = "llvm.loongarch.lasx.xbnz.w"]
    fn __lasx_xbnz_w(a: __v8u32) -> i32;
    #[link_name = "llvm.loongarch.lasx.xbz.b"]
    fn __lasx_xbz_b(a: __v32u8) -> i32;
    #[link_name = "llvm.loongarch.lasx.xbz.d"]
    fn __lasx_xbz_d(a: __v4u64) -> i32;
    #[link_name = "llvm.loongarch.lasx.xbz.h"]
    fn __lasx_xbz_h(a: __v16u16) -> i32;
    #[link_name = "llvm.loongarch.lasx.xbz.v"]
    fn __lasx_xbz_v(a: __v32u8) -> i32;
    #[link_name = "llvm.loongarch.lasx.xbz.w"]
    fn __lasx_xbz_w(a: __v8u32) -> i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.caf.d"]
    fn __lasx_xvfcmp_caf_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.caf.s"]
    fn __lasx_xvfcmp_caf_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.ceq.d"]
    fn __lasx_xvfcmp_ceq_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.ceq.s"]
    fn __lasx_xvfcmp_ceq_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cle.d"]
    fn __lasx_xvfcmp_cle_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cle.s"]
    fn __lasx_xvfcmp_cle_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.clt.d"]
    fn __lasx_xvfcmp_clt_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.clt.s"]
    fn __lasx_xvfcmp_clt_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cne.d"]
    fn __lasx_xvfcmp_cne_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cne.s"]
    fn __lasx_xvfcmp_cne_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cor.d"]
    fn __lasx_xvfcmp_cor_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cor.s"]
    fn __lasx_xvfcmp_cor_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cueq.d"]
    fn __lasx_xvfcmp_cueq_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cueq.s"]
    fn __lasx_xvfcmp_cueq_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cule.d"]
    fn __lasx_xvfcmp_cule_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cule.s"]
    fn __lasx_xvfcmp_cule_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cult.d"]
    fn __lasx_xvfcmp_cult_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cult.s"]
    fn __lasx_xvfcmp_cult_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cun.d"]
    fn __lasx_xvfcmp_cun_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cune.d"]
    fn __lasx_xvfcmp_cune_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cune.s"]
    fn __lasx_xvfcmp_cune_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.cun.s"]
    fn __lasx_xvfcmp_cun_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.saf.d"]
    fn __lasx_xvfcmp_saf_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.saf.s"]
    fn __lasx_xvfcmp_saf_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.seq.d"]
    fn __lasx_xvfcmp_seq_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.seq.s"]
    fn __lasx_xvfcmp_seq_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sle.d"]
    fn __lasx_xvfcmp_sle_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sle.s"]
    fn __lasx_xvfcmp_sle_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.slt.d"]
    fn __lasx_xvfcmp_slt_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.slt.s"]
    fn __lasx_xvfcmp_slt_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sne.d"]
    fn __lasx_xvfcmp_sne_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sne.s"]
    fn __lasx_xvfcmp_sne_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sor.d"]
    fn __lasx_xvfcmp_sor_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sor.s"]
    fn __lasx_xvfcmp_sor_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sueq.d"]
    fn __lasx_xvfcmp_sueq_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sueq.s"]
    fn __lasx_xvfcmp_sueq_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sule.d"]
    fn __lasx_xvfcmp_sule_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sule.s"]
    fn __lasx_xvfcmp_sule_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sult.d"]
    fn __lasx_xvfcmp_sult_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sult.s"]
    fn __lasx_xvfcmp_sult_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sun.d"]
    fn __lasx_xvfcmp_sun_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sune.d"]
    fn __lasx_xvfcmp_sune_d(a: __v4f64, b: __v4f64) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sune.s"]
    fn __lasx_xvfcmp_sune_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvfcmp.sun.s"]
    fn __lasx_xvfcmp_sun_s(a: __v8f32, b: __v8f32) -> __v8i32;
    #[link_name = "llvm.loongarch.lasx.xvpickve.d.f"]
    fn __lasx_xvpickve_d_f(a: __v4f64, b: u32) -> __v4f64;
    #[link_name = "llvm.loongarch.lasx.xvpickve.w.f"]
    fn __lasx_xvpickve_w_f(a: __v8f32, b: u32) -> __v8f32;
    #[link_name = "llvm.loongarch.lasx.xvrepli.b"]
    fn __lasx_xvrepli_b(a: i32) -> __v32i8;
    #[link_name = "llvm.loongarch.lasx.xvrepli.d"]
    fn __lasx_xvrepli_d(a: i32) -> __v4i64;
    #[link_name = "llvm.loongarch.lasx.xvrepli.h"]
    fn __lasx_xvrepli_h(a: i32) -> __v16i16;
    #[link_name = "llvm.loongarch.lasx.xvrepli.w"]
    fn __lasx_xvrepli_w(a: i32) -> __v8i32;
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=lasx_xvsll_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsll_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsll_b(transmute(a), transmute(b))) }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=lasx_xvsll_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsll_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsll_h(transmute(a), transmute(b))) }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=lasx_xvsll_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsll_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsll_w(transmute(a), transmute(b))) }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=lasx_xvsll_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsll_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsll_d(transmute(a), transmute(b))) }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=lasx_xvslli_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslli_b<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvslli_b(transmute(a), IMM3)) }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=lasx_xvslli_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslli_h<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvslli_h(transmute(a), IMM4)) }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=lasx_xvslli_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslli_w<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvslli_w(transmute(a), IMM5)) }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=lasx_xvslli_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslli_d<const IMM6: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvslli_d(transmute(a), IMM6)) }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=lasx_xvsra_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsra_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsra_b(transmute(a), transmute(b))) }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=lasx_xvsra_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsra_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsra_h(transmute(a), transmute(b))) }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=lasx_xvsra_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsra_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsra_w(transmute(a), transmute(b))) }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=lasx_xvsra_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsra_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsra_d(transmute(a), transmute(b))) }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=lasx_xvsrai_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrai_b<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvsrai_b(transmute(a), IMM3)) }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=lasx_xvsrai_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrai_h<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsrai_h(transmute(a), IMM4)) }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=lasx_xvsrai_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrai_w<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsrai_w(transmute(a), IMM5)) }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=lasx_xvsrai_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrai_d<const IMM6: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvsrai_d(transmute(a), IMM6)) }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=lasx_xvsrar_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrar_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrar_b(transmute(a), transmute(b))) }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=lasx_xvsrar_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrar_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrar_h(transmute(a), transmute(b))) }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=lasx_xvsrar_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrar_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrar_w(transmute(a), transmute(b))) }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=lasx_xvsrar_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrar_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrar_d(transmute(a), transmute(b))) }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=lasx_xvsrari_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrari_b<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvsrari_b(transmute(a), IMM3)) }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=lasx_xvsrari_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrari_h<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsrari_h(transmute(a), IMM4)) }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=lasx_xvsrari_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrari_w<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsrari_w(transmute(a), IMM5)) }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=lasx_xvsrari_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrari_d<const IMM6: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvsrari_d(transmute(a), IMM6)) }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=lasx_xvsrl_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrl_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrl_b(transmute(a), transmute(b))) }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=lasx_xvsrl_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrl_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrl_h(transmute(a), transmute(b))) }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=lasx_xvsrl_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrl_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrl_w(transmute(a), transmute(b))) }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=lasx_xvsrl_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrl_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrl_d(transmute(a), transmute(b))) }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=lasx_xvsrli_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrli_b<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvsrli_b(transmute(a), IMM3)) }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=lasx_xvsrli_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrli_h<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsrli_h(transmute(a), IMM4)) }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=lasx_xvsrli_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrli_w<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsrli_w(transmute(a), IMM5)) }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=lasx_xvsrli_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrli_d<const IMM6: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvsrli_d(transmute(a), IMM6)) }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=lasx_xvsrlr_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlr_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrlr_b(transmute(a), transmute(b))) }
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=lasx_xvsrlr_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlr_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrlr_h(transmute(a), transmute(b))) }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=lasx_xvsrlr_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlr_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrlr_w(transmute(a), transmute(b))) }
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=FUNCTION | NAME=lasx_xvsrlr_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlr_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrlr_d(transmute(a), transmute(b))) }
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=lasx_xvsrlri_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlri_b<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvsrlri_b(transmute(a), IMM3)) }
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=lasx_xvsrlri_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlri_h<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsrlri_h(transmute(a), IMM4)) }
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=lasx_xvsrlri_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlri_w<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsrlri_w(transmute(a), IMM5)) }
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=lasx_xvsrlri_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlri_d<const IMM6: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvsrlri_d(transmute(a), IMM6)) }
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=lasx_xvbitclr_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitclr_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitclr_b(transmute(a), transmute(b))) }
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=lasx_xvbitclr_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitclr_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitclr_h(transmute(a), transmute(b))) }
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=FUNCTION | NAME=lasx_xvbitclr_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitclr_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitclr_w(transmute(a), transmute(b))) }
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=lasx_xvbitclr_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitclr_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitclr_d(transmute(a), transmute(b))) }
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=lasx_xvbitclri_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitclri_b<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvbitclri_b(transmute(a), IMM3)) }
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=lasx_xvbitclri_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitclri_h<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvbitclri_h(transmute(a), IMM4)) }
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=lasx_xvbitclri_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitclri_w<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvbitclri_w(transmute(a), IMM5)) }
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=lasx_xvbitclri_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitclri_d<const IMM6: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvbitclri_d(transmute(a), IMM6)) }
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=FUNCTION | NAME=lasx_xvbitset_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitset_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitset_b(transmute(a), transmute(b))) }
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=FUNCTION | NAME=lasx_xvbitset_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitset_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitset_h(transmute(a), transmute(b))) }
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=FUNCTION | NAME=lasx_xvbitset_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitset_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitset_w(transmute(a), transmute(b))) }
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=lasx_xvbitset_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitset_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitset_d(transmute(a), transmute(b))) }
}
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=FUNCTION | NAME=lasx_xvbitseti_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitseti_b<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvbitseti_b(transmute(a), IMM3)) }
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=FUNCTION | NAME=lasx_xvbitseti_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitseti_h<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvbitseti_h(transmute(a), IMM4)) }
}
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=FUNCTION | NAME=lasx_xvbitseti_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitseti_w<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvbitseti_w(transmute(a), IMM5)) }
}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=FUNCTION | NAME=lasx_xvbitseti_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitseti_d<const IMM6: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvbitseti_d(transmute(a), IMM6)) }
}
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=FUNCTION | NAME=lasx_xvbitrev_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitrev_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitrev_b(transmute(a), transmute(b))) }
}
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=FUNCTION | NAME=lasx_xvbitrev_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitrev_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitrev_h(transmute(a), transmute(b))) }
}
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=FUNCTION | NAME=lasx_xvbitrev_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitrev_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitrev_w(transmute(a), transmute(b))) }
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=FUNCTION | NAME=lasx_xvbitrev_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitrev_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitrev_d(transmute(a), transmute(b))) }
}
```

## Block 62
**Metadata**: AST_ID=62 | TYPE=FUNCTION | NAME=lasx_xvbitrevi_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitrevi_b<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvbitrevi_b(transmute(a), IMM3)) }
}
```

## Block 63
**Metadata**: AST_ID=63 | TYPE=FUNCTION | NAME=lasx_xvbitrevi_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitrevi_h<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvbitrevi_h(transmute(a), IMM4)) }
}
```

## Block 64
**Metadata**: AST_ID=64 | TYPE=FUNCTION | NAME=lasx_xvbitrevi_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitrevi_w<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvbitrevi_w(transmute(a), IMM5)) }
}
```

## Block 65
**Metadata**: AST_ID=65 | TYPE=FUNCTION | NAME=lasx_xvbitrevi_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitrevi_d<const IMM6: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvbitrevi_d(transmute(a), IMM6)) }
}
```

## Block 66
**Metadata**: AST_ID=66 | TYPE=FUNCTION | NAME=lasx_xvadd_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvadd_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvadd_b(transmute(a), transmute(b))) }
}
```

## Block 67
**Metadata**: AST_ID=67 | TYPE=FUNCTION | NAME=lasx_xvadd_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvadd_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvadd_h(transmute(a), transmute(b))) }
}
```

## Block 68
**Metadata**: AST_ID=68 | TYPE=FUNCTION | NAME=lasx_xvadd_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvadd_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvadd_w(transmute(a), transmute(b))) }
}
```

## Block 69
**Metadata**: AST_ID=69 | TYPE=FUNCTION | NAME=lasx_xvadd_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvadd_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvadd_d(transmute(a), transmute(b))) }
}
```

## Block 70
**Metadata**: AST_ID=70 | TYPE=FUNCTION | NAME=lasx_xvaddi_bu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddi_bu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvaddi_bu(transmute(a), IMM5)) }
}
```

## Block 71
**Metadata**: AST_ID=71 | TYPE=FUNCTION | NAME=lasx_xvaddi_hu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddi_hu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvaddi_hu(transmute(a), IMM5)) }
}
```

## Block 72
**Metadata**: AST_ID=72 | TYPE=FUNCTION | NAME=lasx_xvaddi_wu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddi_wu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvaddi_wu(transmute(a), IMM5)) }
}
```

## Block 73
**Metadata**: AST_ID=73 | TYPE=FUNCTION | NAME=lasx_xvaddi_du | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddi_du<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvaddi_du(transmute(a), IMM5)) }
}
```

## Block 74
**Metadata**: AST_ID=74 | TYPE=FUNCTION | NAME=lasx_xvsub_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsub_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsub_b(transmute(a), transmute(b))) }
}
```

## Block 75
**Metadata**: AST_ID=75 | TYPE=FUNCTION | NAME=lasx_xvsub_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsub_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsub_h(transmute(a), transmute(b))) }
}
```

## Block 76
**Metadata**: AST_ID=76 | TYPE=FUNCTION | NAME=lasx_xvsub_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsub_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsub_w(transmute(a), transmute(b))) }
}
```

## Block 77
**Metadata**: AST_ID=77 | TYPE=FUNCTION | NAME=lasx_xvsub_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsub_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsub_d(transmute(a), transmute(b))) }
}
```

## Block 78
**Metadata**: AST_ID=78 | TYPE=FUNCTION | NAME=lasx_xvsubi_bu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubi_bu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsubi_bu(transmute(a), IMM5)) }
}
```

## Block 79
**Metadata**: AST_ID=79 | TYPE=FUNCTION | NAME=lasx_xvsubi_hu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubi_hu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsubi_hu(transmute(a), IMM5)) }
}
```

## Block 80
**Metadata**: AST_ID=80 | TYPE=FUNCTION | NAME=lasx_xvsubi_wu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubi_wu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsubi_wu(transmute(a), IMM5)) }
}
```

## Block 81
**Metadata**: AST_ID=81 | TYPE=FUNCTION | NAME=lasx_xvsubi_du | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubi_du<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsubi_du(transmute(a), IMM5)) }
}
```

## Block 82
**Metadata**: AST_ID=82 | TYPE=FUNCTION | NAME=lasx_xvmax_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmax_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmax_b(transmute(a), transmute(b))) }
}
```

## Block 83
**Metadata**: AST_ID=83 | TYPE=FUNCTION | NAME=lasx_xvmax_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmax_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmax_h(transmute(a), transmute(b))) }
}
```

## Block 84
**Metadata**: AST_ID=84 | TYPE=FUNCTION | NAME=lasx_xvmax_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmax_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmax_w(transmute(a), transmute(b))) }
}
```

## Block 85
**Metadata**: AST_ID=85 | TYPE=FUNCTION | NAME=lasx_xvmax_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmax_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmax_d(transmute(a), transmute(b))) }
}
```

## Block 86
**Metadata**: AST_ID=86 | TYPE=FUNCTION | NAME=lasx_xvmaxi_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaxi_b<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvmaxi_b(transmute(a), IMM_S5)) }
}
```

## Block 87
**Metadata**: AST_ID=87 | TYPE=FUNCTION | NAME=lasx_xvmaxi_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaxi_h<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvmaxi_h(transmute(a), IMM_S5)) }
}
```

## Block 88
**Metadata**: AST_ID=88 | TYPE=FUNCTION | NAME=lasx_xvmaxi_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaxi_w<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvmaxi_w(transmute(a), IMM_S5)) }
}
```

## Block 89
**Metadata**: AST_ID=89 | TYPE=FUNCTION | NAME=lasx_xvmaxi_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaxi_d<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvmaxi_d(transmute(a), IMM_S5)) }
}
```

## Block 90
**Metadata**: AST_ID=90 | TYPE=FUNCTION | NAME=lasx_xvmax_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmax_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmax_bu(transmute(a), transmute(b))) }
}
```

## Block 91
**Metadata**: AST_ID=91 | TYPE=FUNCTION | NAME=lasx_xvmax_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmax_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmax_hu(transmute(a), transmute(b))) }
}
```

## Block 92
**Metadata**: AST_ID=92 | TYPE=FUNCTION | NAME=lasx_xvmax_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmax_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmax_wu(transmute(a), transmute(b))) }
}
```

## Block 93
**Metadata**: AST_ID=93 | TYPE=FUNCTION | NAME=lasx_xvmax_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmax_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmax_du(transmute(a), transmute(b))) }
}
```

## Block 94
**Metadata**: AST_ID=94 | TYPE=FUNCTION | NAME=lasx_xvmaxi_bu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaxi_bu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvmaxi_bu(transmute(a), IMM5)) }
}
```

## Block 95
**Metadata**: AST_ID=95 | TYPE=FUNCTION | NAME=lasx_xvmaxi_hu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaxi_hu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvmaxi_hu(transmute(a), IMM5)) }
}
```

## Block 96
**Metadata**: AST_ID=96 | TYPE=FUNCTION | NAME=lasx_xvmaxi_wu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaxi_wu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvmaxi_wu(transmute(a), IMM5)) }
}
```

## Block 97
**Metadata**: AST_ID=97 | TYPE=FUNCTION | NAME=lasx_xvmaxi_du | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaxi_du<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvmaxi_du(transmute(a), IMM5)) }
}
```

## Block 98
**Metadata**: AST_ID=98 | TYPE=FUNCTION | NAME=lasx_xvmin_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmin_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmin_b(transmute(a), transmute(b))) }
}
```

## Block 99
**Metadata**: AST_ID=99 | TYPE=FUNCTION | NAME=lasx_xvmin_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmin_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmin_h(transmute(a), transmute(b))) }
}
```

## Block 100
**Metadata**: AST_ID=100 | TYPE=FUNCTION | NAME=lasx_xvmin_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmin_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmin_w(transmute(a), transmute(b))) }
}
```

## Block 101
**Metadata**: AST_ID=101 | TYPE=FUNCTION | NAME=lasx_xvmin_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmin_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmin_d(transmute(a), transmute(b))) }
}
```

## Block 102
**Metadata**: AST_ID=102 | TYPE=FUNCTION | NAME=lasx_xvmini_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmini_b<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvmini_b(transmute(a), IMM_S5)) }
}
```

## Block 103
**Metadata**: AST_ID=103 | TYPE=FUNCTION | NAME=lasx_xvmini_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmini_h<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvmini_h(transmute(a), IMM_S5)) }
}
```

## Block 104
**Metadata**: AST_ID=104 | TYPE=FUNCTION | NAME=lasx_xvmini_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmini_w<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvmini_w(transmute(a), IMM_S5)) }
}
```

## Block 105
**Metadata**: AST_ID=105 | TYPE=FUNCTION | NAME=lasx_xvmini_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmini_d<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvmini_d(transmute(a), IMM_S5)) }
}
```

## Block 106
**Metadata**: AST_ID=106 | TYPE=FUNCTION | NAME=lasx_xvmin_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmin_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmin_bu(transmute(a), transmute(b))) }
}
```

## Block 107
**Metadata**: AST_ID=107 | TYPE=FUNCTION | NAME=lasx_xvmin_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmin_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmin_hu(transmute(a), transmute(b))) }
}
```

## Block 108
**Metadata**: AST_ID=108 | TYPE=FUNCTION | NAME=lasx_xvmin_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmin_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmin_wu(transmute(a), transmute(b))) }
}
```

## Block 109
**Metadata**: AST_ID=109 | TYPE=FUNCTION | NAME=lasx_xvmin_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmin_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmin_du(transmute(a), transmute(b))) }
}
```

## Block 110
**Metadata**: AST_ID=110 | TYPE=FUNCTION | NAME=lasx_xvmini_bu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmini_bu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvmini_bu(transmute(a), IMM5)) }
}
```

## Block 111
**Metadata**: AST_ID=111 | TYPE=FUNCTION | NAME=lasx_xvmini_hu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmini_hu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvmini_hu(transmute(a), IMM5)) }
}
```

## Block 112
**Metadata**: AST_ID=112 | TYPE=FUNCTION | NAME=lasx_xvmini_wu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmini_wu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvmini_wu(transmute(a), IMM5)) }
}
```

## Block 113
**Metadata**: AST_ID=113 | TYPE=FUNCTION | NAME=lasx_xvmini_du | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmini_du<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvmini_du(transmute(a), IMM5)) }
}
```

## Block 114
**Metadata**: AST_ID=114 | TYPE=FUNCTION | NAME=lasx_xvseq_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvseq_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvseq_b(transmute(a), transmute(b))) }
}
```

## Block 115
**Metadata**: AST_ID=115 | TYPE=FUNCTION | NAME=lasx_xvseq_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvseq_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvseq_h(transmute(a), transmute(b))) }
}
```

## Block 116
**Metadata**: AST_ID=116 | TYPE=FUNCTION | NAME=lasx_xvseq_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvseq_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvseq_w(transmute(a), transmute(b))) }
}
```

## Block 117
**Metadata**: AST_ID=117 | TYPE=FUNCTION | NAME=lasx_xvseq_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvseq_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvseq_d(transmute(a), transmute(b))) }
}
```

## Block 118
**Metadata**: AST_ID=118 | TYPE=FUNCTION | NAME=lasx_xvseqi_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvseqi_b<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvseqi_b(transmute(a), IMM_S5)) }
}
```

## Block 119
**Metadata**: AST_ID=119 | TYPE=FUNCTION | NAME=lasx_xvseqi_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvseqi_h<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvseqi_h(transmute(a), IMM_S5)) }
}
```

## Block 120
**Metadata**: AST_ID=120 | TYPE=FUNCTION | NAME=lasx_xvseqi_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvseqi_w<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvseqi_w(transmute(a), IMM_S5)) }
}
```

## Block 121
**Metadata**: AST_ID=121 | TYPE=FUNCTION | NAME=lasx_xvseqi_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvseqi_d<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvseqi_d(transmute(a), IMM_S5)) }
}
```

## Block 122
**Metadata**: AST_ID=122 | TYPE=FUNCTION | NAME=lasx_xvslt_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslt_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvslt_b(transmute(a), transmute(b))) }
}
```

## Block 123
**Metadata**: AST_ID=123 | TYPE=FUNCTION | NAME=lasx_xvslt_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslt_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvslt_h(transmute(a), transmute(b))) }
}
```

## Block 124
**Metadata**: AST_ID=124 | TYPE=FUNCTION | NAME=lasx_xvslt_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslt_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvslt_w(transmute(a), transmute(b))) }
}
```

## Block 125
**Metadata**: AST_ID=125 | TYPE=FUNCTION | NAME=lasx_xvslt_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslt_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvslt_d(transmute(a), transmute(b))) }
}
```

## Block 126
**Metadata**: AST_ID=126 | TYPE=FUNCTION | NAME=lasx_xvslti_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslti_b<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvslti_b(transmute(a), IMM_S5)) }
}
```

## Block 127
**Metadata**: AST_ID=127 | TYPE=FUNCTION | NAME=lasx_xvslti_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslti_h<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvslti_h(transmute(a), IMM_S5)) }
}
```

## Block 128
**Metadata**: AST_ID=128 | TYPE=FUNCTION | NAME=lasx_xvslti_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslti_w<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvslti_w(transmute(a), IMM_S5)) }
}
```

## Block 129
**Metadata**: AST_ID=129 | TYPE=FUNCTION | NAME=lasx_xvslti_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslti_d<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvslti_d(transmute(a), IMM_S5)) }
}
```

## Block 130
**Metadata**: AST_ID=130 | TYPE=FUNCTION | NAME=lasx_xvslt_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslt_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvslt_bu(transmute(a), transmute(b))) }
}
```

## Block 131
**Metadata**: AST_ID=131 | TYPE=FUNCTION | NAME=lasx_xvslt_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslt_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvslt_hu(transmute(a), transmute(b))) }
}
```

## Block 132
**Metadata**: AST_ID=132 | TYPE=FUNCTION | NAME=lasx_xvslt_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslt_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvslt_wu(transmute(a), transmute(b))) }
}
```

## Block 133
**Metadata**: AST_ID=133 | TYPE=FUNCTION | NAME=lasx_xvslt_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslt_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvslt_du(transmute(a), transmute(b))) }
}
```

## Block 134
**Metadata**: AST_ID=134 | TYPE=FUNCTION | NAME=lasx_xvslti_bu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslti_bu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvslti_bu(transmute(a), IMM5)) }
}
```

## Block 135
**Metadata**: AST_ID=135 | TYPE=FUNCTION | NAME=lasx_xvslti_hu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslti_hu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvslti_hu(transmute(a), IMM5)) }
}
```

## Block 136
**Metadata**: AST_ID=136 | TYPE=FUNCTION | NAME=lasx_xvslti_wu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslti_wu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvslti_wu(transmute(a), IMM5)) }
}
```

## Block 137
**Metadata**: AST_ID=137 | TYPE=FUNCTION | NAME=lasx_xvslti_du | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslti_du<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvslti_du(transmute(a), IMM5)) }
}
```

## Block 138
**Metadata**: AST_ID=138 | TYPE=FUNCTION | NAME=lasx_xvsle_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsle_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsle_b(transmute(a), transmute(b))) }
}
```

## Block 139
**Metadata**: AST_ID=139 | TYPE=FUNCTION | NAME=lasx_xvsle_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsle_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsle_h(transmute(a), transmute(b))) }
}
```

## Block 140
**Metadata**: AST_ID=140 | TYPE=FUNCTION | NAME=lasx_xvsle_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsle_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsle_w(transmute(a), transmute(b))) }
}
```

## Block 141
**Metadata**: AST_ID=141 | TYPE=FUNCTION | NAME=lasx_xvsle_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsle_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsle_d(transmute(a), transmute(b))) }
}
```

## Block 142
**Metadata**: AST_ID=142 | TYPE=FUNCTION | NAME=lasx_xvslei_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslei_b<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvslei_b(transmute(a), IMM_S5)) }
}
```

## Block 143
**Metadata**: AST_ID=143 | TYPE=FUNCTION | NAME=lasx_xvslei_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslei_h<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvslei_h(transmute(a), IMM_S5)) }
}
```

## Block 144
**Metadata**: AST_ID=144 | TYPE=FUNCTION | NAME=lasx_xvslei_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslei_w<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvslei_w(transmute(a), IMM_S5)) }
}
```

## Block 145
**Metadata**: AST_ID=145 | TYPE=FUNCTION | NAME=lasx_xvslei_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslei_d<const IMM_S5: i32>(a: m256i) -> m256i {
    static_assert_simm_bits!(IMM_S5, 5);
    unsafe { transmute(__lasx_xvslei_d(transmute(a), IMM_S5)) }
}
```

## Block 146
**Metadata**: AST_ID=146 | TYPE=FUNCTION | NAME=lasx_xvsle_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsle_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsle_bu(transmute(a), transmute(b))) }
}
```

## Block 147
**Metadata**: AST_ID=147 | TYPE=FUNCTION | NAME=lasx_xvsle_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsle_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsle_hu(transmute(a), transmute(b))) }
}
```

## Block 148
**Metadata**: AST_ID=148 | TYPE=FUNCTION | NAME=lasx_xvsle_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsle_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsle_wu(transmute(a), transmute(b))) }
}
```

## Block 149
**Metadata**: AST_ID=149 | TYPE=FUNCTION | NAME=lasx_xvsle_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsle_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsle_du(transmute(a), transmute(b))) }
}
```

## Block 150
**Metadata**: AST_ID=150 | TYPE=FUNCTION | NAME=lasx_xvslei_bu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslei_bu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvslei_bu(transmute(a), IMM5)) }
}
```

## Block 151
**Metadata**: AST_ID=151 | TYPE=FUNCTION | NAME=lasx_xvslei_hu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslei_hu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvslei_hu(transmute(a), IMM5)) }
}
```

## Block 152
**Metadata**: AST_ID=152 | TYPE=FUNCTION | NAME=lasx_xvslei_wu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslei_wu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvslei_wu(transmute(a), IMM5)) }
}
```

## Block 153
**Metadata**: AST_ID=153 | TYPE=FUNCTION | NAME=lasx_xvslei_du | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvslei_du<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvslei_du(transmute(a), IMM5)) }
}
```

## Block 154
**Metadata**: AST_ID=154 | TYPE=FUNCTION | NAME=lasx_xvsat_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsat_b<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvsat_b(transmute(a), IMM3)) }
}
```

## Block 155
**Metadata**: AST_ID=155 | TYPE=FUNCTION | NAME=lasx_xvsat_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsat_h<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsat_h(transmute(a), IMM4)) }
}
```

## Block 156
**Metadata**: AST_ID=156 | TYPE=FUNCTION | NAME=lasx_xvsat_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsat_w<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsat_w(transmute(a), IMM5)) }
}
```

## Block 157
**Metadata**: AST_ID=157 | TYPE=FUNCTION | NAME=lasx_xvsat_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsat_d<const IMM6: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvsat_d(transmute(a), IMM6)) }
}
```

## Block 158
**Metadata**: AST_ID=158 | TYPE=FUNCTION | NAME=lasx_xvsat_bu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsat_bu<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvsat_bu(transmute(a), IMM3)) }
}
```

## Block 159
**Metadata**: AST_ID=159 | TYPE=FUNCTION | NAME=lasx_xvsat_hu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsat_hu<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsat_hu(transmute(a), IMM4)) }
}
```

## Block 160
**Metadata**: AST_ID=160 | TYPE=FUNCTION | NAME=lasx_xvsat_wu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsat_wu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsat_wu(transmute(a), IMM5)) }
}
```

## Block 161
**Metadata**: AST_ID=161 | TYPE=FUNCTION | NAME=lasx_xvsat_du | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsat_du<const IMM6: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvsat_du(transmute(a), IMM6)) }
}
```

## Block 162
**Metadata**: AST_ID=162 | TYPE=FUNCTION | NAME=lasx_xvadda_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvadda_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvadda_b(transmute(a), transmute(b))) }
}
```

## Block 163
**Metadata**: AST_ID=163 | TYPE=FUNCTION | NAME=lasx_xvadda_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvadda_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvadda_h(transmute(a), transmute(b))) }
}
```

## Block 164
**Metadata**: AST_ID=164 | TYPE=FUNCTION | NAME=lasx_xvadda_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvadda_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvadda_w(transmute(a), transmute(b))) }
}
```

## Block 165
**Metadata**: AST_ID=165 | TYPE=FUNCTION | NAME=lasx_xvadda_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvadda_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvadda_d(transmute(a), transmute(b))) }
}
```

## Block 166
**Metadata**: AST_ID=166 | TYPE=FUNCTION | NAME=lasx_xvsadd_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsadd_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsadd_b(transmute(a), transmute(b))) }
}
```

## Block 167
**Metadata**: AST_ID=167 | TYPE=FUNCTION | NAME=lasx_xvsadd_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsadd_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsadd_h(transmute(a), transmute(b))) }
}
```

## Block 168
**Metadata**: AST_ID=168 | TYPE=FUNCTION | NAME=lasx_xvsadd_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsadd_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsadd_w(transmute(a), transmute(b))) }
}
```

## Block 169
**Metadata**: AST_ID=169 | TYPE=FUNCTION | NAME=lasx_xvsadd_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsadd_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsadd_d(transmute(a), transmute(b))) }
}
```

## Block 170
**Metadata**: AST_ID=170 | TYPE=FUNCTION | NAME=lasx_xvsadd_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsadd_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsadd_bu(transmute(a), transmute(b))) }
}
```

## Block 171
**Metadata**: AST_ID=171 | TYPE=FUNCTION | NAME=lasx_xvsadd_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsadd_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsadd_hu(transmute(a), transmute(b))) }
}
```

## Block 172
**Metadata**: AST_ID=172 | TYPE=FUNCTION | NAME=lasx_xvsadd_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsadd_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsadd_wu(transmute(a), transmute(b))) }
}
```

## Block 173
**Metadata**: AST_ID=173 | TYPE=FUNCTION | NAME=lasx_xvsadd_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsadd_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsadd_du(transmute(a), transmute(b))) }
}
```

## Block 174
**Metadata**: AST_ID=174 | TYPE=FUNCTION | NAME=lasx_xvavg_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavg_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavg_b(transmute(a), transmute(b))) }
}
```

## Block 175
**Metadata**: AST_ID=175 | TYPE=FUNCTION | NAME=lasx_xvavg_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavg_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavg_h(transmute(a), transmute(b))) }
}
```

## Block 176
**Metadata**: AST_ID=176 | TYPE=FUNCTION | NAME=lasx_xvavg_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavg_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavg_w(transmute(a), transmute(b))) }
}
```

## Block 177
**Metadata**: AST_ID=177 | TYPE=FUNCTION | NAME=lasx_xvavg_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavg_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavg_d(transmute(a), transmute(b))) }
}
```

## Block 178
**Metadata**: AST_ID=178 | TYPE=FUNCTION | NAME=lasx_xvavg_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavg_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavg_bu(transmute(a), transmute(b))) }
}
```

## Block 179
**Metadata**: AST_ID=179 | TYPE=FUNCTION | NAME=lasx_xvavg_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavg_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavg_hu(transmute(a), transmute(b))) }
}
```

## Block 180
**Metadata**: AST_ID=180 | TYPE=FUNCTION | NAME=lasx_xvavg_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavg_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavg_wu(transmute(a), transmute(b))) }
}
```

## Block 181
**Metadata**: AST_ID=181 | TYPE=FUNCTION | NAME=lasx_xvavg_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavg_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavg_du(transmute(a), transmute(b))) }
}
```

## Block 182
**Metadata**: AST_ID=182 | TYPE=FUNCTION | NAME=lasx_xvavgr_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavgr_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavgr_b(transmute(a), transmute(b))) }
}
```

## Block 183
**Metadata**: AST_ID=183 | TYPE=FUNCTION | NAME=lasx_xvavgr_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavgr_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavgr_h(transmute(a), transmute(b))) }
}
```

## Block 184
**Metadata**: AST_ID=184 | TYPE=FUNCTION | NAME=lasx_xvavgr_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavgr_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavgr_w(transmute(a), transmute(b))) }
}
```

## Block 185
**Metadata**: AST_ID=185 | TYPE=FUNCTION | NAME=lasx_xvavgr_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavgr_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavgr_d(transmute(a), transmute(b))) }
}
```

## Block 186
**Metadata**: AST_ID=186 | TYPE=FUNCTION | NAME=lasx_xvavgr_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavgr_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavgr_bu(transmute(a), transmute(b))) }
}
```

## Block 187
**Metadata**: AST_ID=187 | TYPE=FUNCTION | NAME=lasx_xvavgr_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavgr_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavgr_hu(transmute(a), transmute(b))) }
}
```

## Block 188
**Metadata**: AST_ID=188 | TYPE=FUNCTION | NAME=lasx_xvavgr_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavgr_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavgr_wu(transmute(a), transmute(b))) }
}
```

## Block 189
**Metadata**: AST_ID=189 | TYPE=FUNCTION | NAME=lasx_xvavgr_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvavgr_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvavgr_du(transmute(a), transmute(b))) }
}
```

## Block 190
**Metadata**: AST_ID=190 | TYPE=FUNCTION | NAME=lasx_xvssub_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssub_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssub_b(transmute(a), transmute(b))) }
}
```

## Block 191
**Metadata**: AST_ID=191 | TYPE=FUNCTION | NAME=lasx_xvssub_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssub_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssub_h(transmute(a), transmute(b))) }
}
```

## Block 192
**Metadata**: AST_ID=192 | TYPE=FUNCTION | NAME=lasx_xvssub_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssub_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssub_w(transmute(a), transmute(b))) }
}
```

## Block 193
**Metadata**: AST_ID=193 | TYPE=FUNCTION | NAME=lasx_xvssub_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssub_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssub_d(transmute(a), transmute(b))) }
}
```

## Block 194
**Metadata**: AST_ID=194 | TYPE=FUNCTION | NAME=lasx_xvssub_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssub_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssub_bu(transmute(a), transmute(b))) }
}
```

## Block 195
**Metadata**: AST_ID=195 | TYPE=FUNCTION | NAME=lasx_xvssub_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssub_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssub_hu(transmute(a), transmute(b))) }
}
```

## Block 196
**Metadata**: AST_ID=196 | TYPE=FUNCTION | NAME=lasx_xvssub_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssub_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssub_wu(transmute(a), transmute(b))) }
}
```

## Block 197
**Metadata**: AST_ID=197 | TYPE=FUNCTION | NAME=lasx_xvssub_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssub_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssub_du(transmute(a), transmute(b))) }
}
```

## Block 198
**Metadata**: AST_ID=198 | TYPE=FUNCTION | NAME=lasx_xvabsd_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvabsd_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvabsd_b(transmute(a), transmute(b))) }
}
```

## Block 199
**Metadata**: AST_ID=199 | TYPE=FUNCTION | NAME=lasx_xvabsd_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvabsd_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvabsd_h(transmute(a), transmute(b))) }
}
```

## Block 200
**Metadata**: AST_ID=200 | TYPE=FUNCTION | NAME=lasx_xvabsd_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvabsd_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvabsd_w(transmute(a), transmute(b))) }
}
```

## Block 201
**Metadata**: AST_ID=201 | TYPE=FUNCTION | NAME=lasx_xvabsd_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvabsd_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvabsd_d(transmute(a), transmute(b))) }
}
```

## Block 202
**Metadata**: AST_ID=202 | TYPE=FUNCTION | NAME=lasx_xvabsd_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvabsd_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvabsd_bu(transmute(a), transmute(b))) }
}
```

## Block 203
**Metadata**: AST_ID=203 | TYPE=FUNCTION | NAME=lasx_xvabsd_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvabsd_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvabsd_hu(transmute(a), transmute(b))) }
}
```

## Block 204
**Metadata**: AST_ID=204 | TYPE=FUNCTION | NAME=lasx_xvabsd_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvabsd_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvabsd_wu(transmute(a), transmute(b))) }
}
```

## Block 205
**Metadata**: AST_ID=205 | TYPE=FUNCTION | NAME=lasx_xvabsd_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvabsd_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvabsd_du(transmute(a), transmute(b))) }
}
```

## Block 206
**Metadata**: AST_ID=206 | TYPE=FUNCTION | NAME=lasx_xvmul_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmul_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmul_b(transmute(a), transmute(b))) }
}
```

## Block 207
**Metadata**: AST_ID=207 | TYPE=FUNCTION | NAME=lasx_xvmul_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmul_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmul_h(transmute(a), transmute(b))) }
}
```

## Block 208
**Metadata**: AST_ID=208 | TYPE=FUNCTION | NAME=lasx_xvmul_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmul_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmul_w(transmute(a), transmute(b))) }
}
```

## Block 209
**Metadata**: AST_ID=209 | TYPE=FUNCTION | NAME=lasx_xvmul_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmul_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmul_d(transmute(a), transmute(b))) }
}
```

## Block 210
**Metadata**: AST_ID=210 | TYPE=FUNCTION | NAME=lasx_xvmadd_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmadd_b(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmadd_b(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 211
**Metadata**: AST_ID=211 | TYPE=FUNCTION | NAME=lasx_xvmadd_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmadd_h(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmadd_h(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 212
**Metadata**: AST_ID=212 | TYPE=FUNCTION | NAME=lasx_xvmadd_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmadd_w(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmadd_w(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 213
**Metadata**: AST_ID=213 | TYPE=FUNCTION | NAME=lasx_xvmadd_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmadd_d(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmadd_d(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 214
**Metadata**: AST_ID=214 | TYPE=FUNCTION | NAME=lasx_xvmsub_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmsub_b(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmsub_b(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 215
**Metadata**: AST_ID=215 | TYPE=FUNCTION | NAME=lasx_xvmsub_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmsub_h(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmsub_h(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 216
**Metadata**: AST_ID=216 | TYPE=FUNCTION | NAME=lasx_xvmsub_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmsub_w(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmsub_w(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 217
**Metadata**: AST_ID=217 | TYPE=FUNCTION | NAME=lasx_xvmsub_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmsub_d(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmsub_d(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 218
**Metadata**: AST_ID=218 | TYPE=FUNCTION | NAME=lasx_xvdiv_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvdiv_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvdiv_b(transmute(a), transmute(b))) }
}
```

## Block 219
**Metadata**: AST_ID=219 | TYPE=FUNCTION | NAME=lasx_xvdiv_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvdiv_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvdiv_h(transmute(a), transmute(b))) }
}
```

## Block 220
**Metadata**: AST_ID=220 | TYPE=FUNCTION | NAME=lasx_xvdiv_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvdiv_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvdiv_w(transmute(a), transmute(b))) }
}
```

## Block 221
**Metadata**: AST_ID=221 | TYPE=FUNCTION | NAME=lasx_xvdiv_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvdiv_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvdiv_d(transmute(a), transmute(b))) }
}
```

## Block 222
**Metadata**: AST_ID=222 | TYPE=FUNCTION | NAME=lasx_xvdiv_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvdiv_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvdiv_bu(transmute(a), transmute(b))) }
}
```

## Block 223
**Metadata**: AST_ID=223 | TYPE=FUNCTION | NAME=lasx_xvdiv_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvdiv_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvdiv_hu(transmute(a), transmute(b))) }
}
```

## Block 224
**Metadata**: AST_ID=224 | TYPE=FUNCTION | NAME=lasx_xvdiv_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvdiv_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvdiv_wu(transmute(a), transmute(b))) }
}
```

## Block 225
**Metadata**: AST_ID=225 | TYPE=FUNCTION | NAME=lasx_xvdiv_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvdiv_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvdiv_du(transmute(a), transmute(b))) }
}
```

## Block 226
**Metadata**: AST_ID=226 | TYPE=FUNCTION | NAME=lasx_xvhaddw_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhaddw_h_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhaddw_h_b(transmute(a), transmute(b))) }
}
```

## Block 227
**Metadata**: AST_ID=227 | TYPE=FUNCTION | NAME=lasx_xvhaddw_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhaddw_w_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhaddw_w_h(transmute(a), transmute(b))) }
}
```

## Block 228
**Metadata**: AST_ID=228 | TYPE=FUNCTION | NAME=lasx_xvhaddw_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhaddw_d_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhaddw_d_w(transmute(a), transmute(b))) }
}
```

## Block 229
**Metadata**: AST_ID=229 | TYPE=FUNCTION | NAME=lasx_xvhaddw_hu_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhaddw_hu_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhaddw_hu_bu(transmute(a), transmute(b))) }
}
```

## Block 230
**Metadata**: AST_ID=230 | TYPE=FUNCTION | NAME=lasx_xvhaddw_wu_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhaddw_wu_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhaddw_wu_hu(transmute(a), transmute(b))) }
}
```

## Block 231
**Metadata**: AST_ID=231 | TYPE=FUNCTION | NAME=lasx_xvhaddw_du_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhaddw_du_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhaddw_du_wu(transmute(a), transmute(b))) }
}
```

## Block 232
**Metadata**: AST_ID=232 | TYPE=FUNCTION | NAME=lasx_xvhsubw_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhsubw_h_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhsubw_h_b(transmute(a), transmute(b))) }
}
```

## Block 233
**Metadata**: AST_ID=233 | TYPE=FUNCTION | NAME=lasx_xvhsubw_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhsubw_w_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhsubw_w_h(transmute(a), transmute(b))) }
}
```

## Block 234
**Metadata**: AST_ID=234 | TYPE=FUNCTION | NAME=lasx_xvhsubw_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhsubw_d_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhsubw_d_w(transmute(a), transmute(b))) }
}
```

## Block 235
**Metadata**: AST_ID=235 | TYPE=FUNCTION | NAME=lasx_xvhsubw_hu_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhsubw_hu_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhsubw_hu_bu(transmute(a), transmute(b))) }
}
```

## Block 236
**Metadata**: AST_ID=236 | TYPE=FUNCTION | NAME=lasx_xvhsubw_wu_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhsubw_wu_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhsubw_wu_hu(transmute(a), transmute(b))) }
}
```

## Block 237
**Metadata**: AST_ID=237 | TYPE=FUNCTION | NAME=lasx_xvhsubw_du_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhsubw_du_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhsubw_du_wu(transmute(a), transmute(b))) }
}
```

## Block 238
**Metadata**: AST_ID=238 | TYPE=FUNCTION | NAME=lasx_xvmod_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmod_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmod_b(transmute(a), transmute(b))) }
}
```

## Block 239
**Metadata**: AST_ID=239 | TYPE=FUNCTION | NAME=lasx_xvmod_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmod_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmod_h(transmute(a), transmute(b))) }
}
```

## Block 240
**Metadata**: AST_ID=240 | TYPE=FUNCTION | NAME=lasx_xvmod_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmod_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmod_w(transmute(a), transmute(b))) }
}
```

## Block 241
**Metadata**: AST_ID=241 | TYPE=FUNCTION | NAME=lasx_xvmod_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmod_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmod_d(transmute(a), transmute(b))) }
}
```

## Block 242
**Metadata**: AST_ID=242 | TYPE=FUNCTION | NAME=lasx_xvmod_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmod_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmod_bu(transmute(a), transmute(b))) }
}
```

## Block 243
**Metadata**: AST_ID=243 | TYPE=FUNCTION | NAME=lasx_xvmod_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmod_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmod_hu(transmute(a), transmute(b))) }
}
```

## Block 244
**Metadata**: AST_ID=244 | TYPE=FUNCTION | NAME=lasx_xvmod_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmod_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmod_wu(transmute(a), transmute(b))) }
}
```

## Block 245
**Metadata**: AST_ID=245 | TYPE=FUNCTION | NAME=lasx_xvmod_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmod_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmod_du(transmute(a), transmute(b))) }
}
```

## Block 246
**Metadata**: AST_ID=246 | TYPE=FUNCTION | NAME=lasx_xvrepl128vei_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrepl128vei_b<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvrepl128vei_b(transmute(a), IMM4)) }
}
```

## Block 247
**Metadata**: AST_ID=247 | TYPE=FUNCTION | NAME=lasx_xvrepl128vei_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrepl128vei_h<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvrepl128vei_h(transmute(a), IMM3)) }
}
```

## Block 248
**Metadata**: AST_ID=248 | TYPE=FUNCTION | NAME=lasx_xvrepl128vei_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrepl128vei_w<const IMM2: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM2, 2);
    unsafe { transmute(__lasx_xvrepl128vei_w(transmute(a), IMM2)) }
}
```

## Block 249
**Metadata**: AST_ID=249 | TYPE=FUNCTION | NAME=lasx_xvrepl128vei_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrepl128vei_d<const IMM1: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM1, 1);
    unsafe { transmute(__lasx_xvrepl128vei_d(transmute(a), IMM1)) }
}
```

## Block 250
**Metadata**: AST_ID=250 | TYPE=FUNCTION | NAME=lasx_xvpickev_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickev_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpickev_b(transmute(a), transmute(b))) }
}
```

## Block 251
**Metadata**: AST_ID=251 | TYPE=FUNCTION | NAME=lasx_xvpickev_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickev_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpickev_h(transmute(a), transmute(b))) }
}
```

## Block 252
**Metadata**: AST_ID=252 | TYPE=FUNCTION | NAME=lasx_xvpickev_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickev_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpickev_w(transmute(a), transmute(b))) }
}
```

## Block 253
**Metadata**: AST_ID=253 | TYPE=FUNCTION | NAME=lasx_xvpickev_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickev_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpickev_d(transmute(a), transmute(b))) }
}
```

## Block 254
**Metadata**: AST_ID=254 | TYPE=FUNCTION | NAME=lasx_xvpickod_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickod_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpickod_b(transmute(a), transmute(b))) }
}
```

## Block 255
**Metadata**: AST_ID=255 | TYPE=FUNCTION | NAME=lasx_xvpickod_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickod_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpickod_h(transmute(a), transmute(b))) }
}
```

## Block 256
**Metadata**: AST_ID=256 | TYPE=FUNCTION | NAME=lasx_xvpickod_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickod_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpickod_w(transmute(a), transmute(b))) }
}
```

## Block 257
**Metadata**: AST_ID=257 | TYPE=FUNCTION | NAME=lasx_xvpickod_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickod_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpickod_d(transmute(a), transmute(b))) }
}
```

## Block 258
**Metadata**: AST_ID=258 | TYPE=FUNCTION | NAME=lasx_xvilvh_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvilvh_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvilvh_b(transmute(a), transmute(b))) }
}
```

## Block 259
**Metadata**: AST_ID=259 | TYPE=FUNCTION | NAME=lasx_xvilvh_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvilvh_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvilvh_h(transmute(a), transmute(b))) }
}
```

## Block 260
**Metadata**: AST_ID=260 | TYPE=FUNCTION | NAME=lasx_xvilvh_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvilvh_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvilvh_w(transmute(a), transmute(b))) }
}
```

## Block 261
**Metadata**: AST_ID=261 | TYPE=FUNCTION | NAME=lasx_xvilvh_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvilvh_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvilvh_d(transmute(a), transmute(b))) }
}
```

## Block 262
**Metadata**: AST_ID=262 | TYPE=FUNCTION | NAME=lasx_xvilvl_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvilvl_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvilvl_b(transmute(a), transmute(b))) }
}
```

## Block 263
**Metadata**: AST_ID=263 | TYPE=FUNCTION | NAME=lasx_xvilvl_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvilvl_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvilvl_h(transmute(a), transmute(b))) }
}
```

## Block 264
**Metadata**: AST_ID=264 | TYPE=FUNCTION | NAME=lasx_xvilvl_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvilvl_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvilvl_w(transmute(a), transmute(b))) }
}
```

## Block 265
**Metadata**: AST_ID=265 | TYPE=FUNCTION | NAME=lasx_xvilvl_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvilvl_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvilvl_d(transmute(a), transmute(b))) }
}
```

## Block 266
**Metadata**: AST_ID=266 | TYPE=FUNCTION | NAME=lasx_xvpackev_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpackev_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpackev_b(transmute(a), transmute(b))) }
}
```

## Block 267
**Metadata**: AST_ID=267 | TYPE=FUNCTION | NAME=lasx_xvpackev_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpackev_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpackev_h(transmute(a), transmute(b))) }
}
```

## Block 268
**Metadata**: AST_ID=268 | TYPE=FUNCTION | NAME=lasx_xvpackev_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpackev_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpackev_w(transmute(a), transmute(b))) }
}
```

## Block 269
**Metadata**: AST_ID=269 | TYPE=FUNCTION | NAME=lasx_xvpackev_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpackev_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpackev_d(transmute(a), transmute(b))) }
}
```

## Block 270
**Metadata**: AST_ID=270 | TYPE=FUNCTION | NAME=lasx_xvpackod_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpackod_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpackod_b(transmute(a), transmute(b))) }
}
```

## Block 271
**Metadata**: AST_ID=271 | TYPE=FUNCTION | NAME=lasx_xvpackod_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpackod_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpackod_h(transmute(a), transmute(b))) }
}
```

## Block 272
**Metadata**: AST_ID=272 | TYPE=FUNCTION | NAME=lasx_xvpackod_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpackod_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpackod_w(transmute(a), transmute(b))) }
}
```

## Block 273
**Metadata**: AST_ID=273 | TYPE=FUNCTION | NAME=lasx_xvpackod_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpackod_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpackod_d(transmute(a), transmute(b))) }
}
```

## Block 274
**Metadata**: AST_ID=274 | TYPE=FUNCTION | NAME=lasx_xvshuf_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvshuf_b(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvshuf_b(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 275
**Metadata**: AST_ID=275 | TYPE=FUNCTION | NAME=lasx_xvshuf_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvshuf_h(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvshuf_h(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 276
**Metadata**: AST_ID=276 | TYPE=FUNCTION | NAME=lasx_xvshuf_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvshuf_w(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvshuf_w(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 277
**Metadata**: AST_ID=277 | TYPE=FUNCTION | NAME=lasx_xvshuf_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvshuf_d(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvshuf_d(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 278
**Metadata**: AST_ID=278 | TYPE=FUNCTION | NAME=lasx_xvand_v | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvand_v(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvand_v(transmute(a), transmute(b))) }
}
```

## Block 279
**Metadata**: AST_ID=279 | TYPE=FUNCTION | NAME=lasx_xvandi_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvandi_b<const IMM8: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvandi_b(transmute(a), IMM8)) }
}
```

## Block 280
**Metadata**: AST_ID=280 | TYPE=FUNCTION | NAME=lasx_xvor_v | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvor_v(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvor_v(transmute(a), transmute(b))) }
}
```

## Block 281
**Metadata**: AST_ID=281 | TYPE=FUNCTION | NAME=lasx_xvori_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvori_b<const IMM8: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvori_b(transmute(a), IMM8)) }
}
```

## Block 282
**Metadata**: AST_ID=282 | TYPE=FUNCTION | NAME=lasx_xvnor_v | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvnor_v(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvnor_v(transmute(a), transmute(b))) }
}
```

## Block 283
**Metadata**: AST_ID=283 | TYPE=FUNCTION | NAME=lasx_xvnori_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvnori_b<const IMM8: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvnori_b(transmute(a), IMM8)) }
}
```

## Block 284
**Metadata**: AST_ID=284 | TYPE=FUNCTION | NAME=lasx_xvxor_v | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvxor_v(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvxor_v(transmute(a), transmute(b))) }
}
```

## Block 285
**Metadata**: AST_ID=285 | TYPE=FUNCTION | NAME=lasx_xvxori_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvxori_b<const IMM8: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvxori_b(transmute(a), IMM8)) }
}
```

## Block 286
**Metadata**: AST_ID=286 | TYPE=FUNCTION | NAME=lasx_xvbitsel_v | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitsel_v(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvbitsel_v(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 287
**Metadata**: AST_ID=287 | TYPE=FUNCTION | NAME=lasx_xvbitseli_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbitseli_b<const IMM8: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvbitseli_b(transmute(a), transmute(b), IMM8)) }
}
```

## Block 288
**Metadata**: AST_ID=288 | TYPE=FUNCTION | NAME=lasx_xvshuf4i_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvshuf4i_b<const IMM8: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvshuf4i_b(transmute(a), IMM8)) }
}
```

## Block 289
**Metadata**: AST_ID=289 | TYPE=FUNCTION | NAME=lasx_xvshuf4i_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvshuf4i_h<const IMM8: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvshuf4i_h(transmute(a), IMM8)) }
}
```

## Block 290
**Metadata**: AST_ID=290 | TYPE=FUNCTION | NAME=lasx_xvshuf4i_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvshuf4i_w<const IMM8: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvshuf4i_w(transmute(a), IMM8)) }
}
```

## Block 291
**Metadata**: AST_ID=291 | TYPE=FUNCTION | NAME=lasx_xvreplgr2vr_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplgr2vr_b(a: i32) -> m256i {
    unsafe { transmute(__lasx_xvreplgr2vr_b(transmute(a))) }
}
```

## Block 292
**Metadata**: AST_ID=292 | TYPE=FUNCTION | NAME=lasx_xvreplgr2vr_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplgr2vr_h(a: i32) -> m256i {
    unsafe { transmute(__lasx_xvreplgr2vr_h(transmute(a))) }
}
```

## Block 293
**Metadata**: AST_ID=293 | TYPE=FUNCTION | NAME=lasx_xvreplgr2vr_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplgr2vr_w(a: i32) -> m256i {
    unsafe { transmute(__lasx_xvreplgr2vr_w(transmute(a))) }
}
```

## Block 294
**Metadata**: AST_ID=294 | TYPE=FUNCTION | NAME=lasx_xvreplgr2vr_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplgr2vr_d(a: i64) -> m256i {
    unsafe { transmute(__lasx_xvreplgr2vr_d(transmute(a))) }
}
```

## Block 295
**Metadata**: AST_ID=295 | TYPE=FUNCTION | NAME=lasx_xvpcnt_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpcnt_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpcnt_b(transmute(a))) }
}
```

## Block 296
**Metadata**: AST_ID=296 | TYPE=FUNCTION | NAME=lasx_xvpcnt_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpcnt_h(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpcnt_h(transmute(a))) }
}
```

## Block 297
**Metadata**: AST_ID=297 | TYPE=FUNCTION | NAME=lasx_xvpcnt_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpcnt_w(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpcnt_w(transmute(a))) }
}
```

## Block 298
**Metadata**: AST_ID=298 | TYPE=FUNCTION | NAME=lasx_xvpcnt_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpcnt_d(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvpcnt_d(transmute(a))) }
}
```

## Block 299
**Metadata**: AST_ID=299 | TYPE=FUNCTION | NAME=lasx_xvclo_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvclo_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvclo_b(transmute(a))) }
}
```

## Block 300
**Metadata**: AST_ID=300 | TYPE=FUNCTION | NAME=lasx_xvclo_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvclo_h(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvclo_h(transmute(a))) }
}
```

## Block 301
**Metadata**: AST_ID=301 | TYPE=FUNCTION | NAME=lasx_xvclo_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvclo_w(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvclo_w(transmute(a))) }
}
```

## Block 302
**Metadata**: AST_ID=302 | TYPE=FUNCTION | NAME=lasx_xvclo_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvclo_d(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvclo_d(transmute(a))) }
}
```

## Block 303
**Metadata**: AST_ID=303 | TYPE=FUNCTION | NAME=lasx_xvclz_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvclz_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvclz_b(transmute(a))) }
}
```

## Block 304
**Metadata**: AST_ID=304 | TYPE=FUNCTION | NAME=lasx_xvclz_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvclz_h(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvclz_h(transmute(a))) }
}
```

## Block 305
**Metadata**: AST_ID=305 | TYPE=FUNCTION | NAME=lasx_xvclz_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvclz_w(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvclz_w(transmute(a))) }
}
```

## Block 306
**Metadata**: AST_ID=306 | TYPE=FUNCTION | NAME=lasx_xvclz_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvclz_d(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvclz_d(transmute(a))) }
}
```

## Block 307
**Metadata**: AST_ID=307 | TYPE=FUNCTION | NAME=lasx_xvfadd_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfadd_s(a: m256, b: m256) -> m256 {
    unsafe { transmute(__lasx_xvfadd_s(transmute(a), transmute(b))) }
}
```

## Block 308
**Metadata**: AST_ID=308 | TYPE=FUNCTION | NAME=lasx_xvfadd_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfadd_d(a: m256d, b: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfadd_d(transmute(a), transmute(b))) }
}
```

## Block 309
**Metadata**: AST_ID=309 | TYPE=FUNCTION | NAME=lasx_xvfsub_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfsub_s(a: m256, b: m256) -> m256 {
    unsafe { transmute(__lasx_xvfsub_s(transmute(a), transmute(b))) }
}
```

## Block 310
**Metadata**: AST_ID=310 | TYPE=FUNCTION | NAME=lasx_xvfsub_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfsub_d(a: m256d, b: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfsub_d(transmute(a), transmute(b))) }
}
```

## Block 311
**Metadata**: AST_ID=311 | TYPE=FUNCTION | NAME=lasx_xvfmul_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmul_s(a: m256, b: m256) -> m256 {
    unsafe { transmute(__lasx_xvfmul_s(transmute(a), transmute(b))) }
}
```

## Block 312
**Metadata**: AST_ID=312 | TYPE=FUNCTION | NAME=lasx_xvfmul_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmul_d(a: m256d, b: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfmul_d(transmute(a), transmute(b))) }
}
```

## Block 313
**Metadata**: AST_ID=313 | TYPE=FUNCTION | NAME=lasx_xvfdiv_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfdiv_s(a: m256, b: m256) -> m256 {
    unsafe { transmute(__lasx_xvfdiv_s(transmute(a), transmute(b))) }
}
```

## Block 314
**Metadata**: AST_ID=314 | TYPE=FUNCTION | NAME=lasx_xvfdiv_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfdiv_d(a: m256d, b: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfdiv_d(transmute(a), transmute(b))) }
}
```

## Block 315
**Metadata**: AST_ID=315 | TYPE=FUNCTION | NAME=lasx_xvfcvt_h_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcvt_h_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcvt_h_s(transmute(a), transmute(b))) }
}
```

## Block 316
**Metadata**: AST_ID=316 | TYPE=FUNCTION | NAME=lasx_xvfcvt_s_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcvt_s_d(a: m256d, b: m256d) -> m256 {
    unsafe { transmute(__lasx_xvfcvt_s_d(transmute(a), transmute(b))) }
}
```

## Block 317
**Metadata**: AST_ID=317 | TYPE=FUNCTION | NAME=lasx_xvfmin_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmin_s(a: m256, b: m256) -> m256 {
    unsafe { transmute(__lasx_xvfmin_s(transmute(a), transmute(b))) }
}
```

## Block 318
**Metadata**: AST_ID=318 | TYPE=FUNCTION | NAME=lasx_xvfmin_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmin_d(a: m256d, b: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfmin_d(transmute(a), transmute(b))) }
}
```

## Block 319
**Metadata**: AST_ID=319 | TYPE=FUNCTION | NAME=lasx_xvfmina_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmina_s(a: m256, b: m256) -> m256 {
    unsafe { transmute(__lasx_xvfmina_s(transmute(a), transmute(b))) }
}
```

## Block 320
**Metadata**: AST_ID=320 | TYPE=FUNCTION | NAME=lasx_xvfmina_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmina_d(a: m256d, b: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfmina_d(transmute(a), transmute(b))) }
}
```

## Block 321
**Metadata**: AST_ID=321 | TYPE=FUNCTION | NAME=lasx_xvfmax_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmax_s(a: m256, b: m256) -> m256 {
    unsafe { transmute(__lasx_xvfmax_s(transmute(a), transmute(b))) }
}
```

## Block 322
**Metadata**: AST_ID=322 | TYPE=FUNCTION | NAME=lasx_xvfmax_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmax_d(a: m256d, b: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfmax_d(transmute(a), transmute(b))) }
}
```

## Block 323
**Metadata**: AST_ID=323 | TYPE=FUNCTION | NAME=lasx_xvfmaxa_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmaxa_s(a: m256, b: m256) -> m256 {
    unsafe { transmute(__lasx_xvfmaxa_s(transmute(a), transmute(b))) }
}
```

## Block 324
**Metadata**: AST_ID=324 | TYPE=FUNCTION | NAME=lasx_xvfmaxa_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmaxa_d(a: m256d, b: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfmaxa_d(transmute(a), transmute(b))) }
}
```

## Block 325
**Metadata**: AST_ID=325 | TYPE=FUNCTION | NAME=lasx_xvfclass_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfclass_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvfclass_s(transmute(a))) }
}
```

## Block 326
**Metadata**: AST_ID=326 | TYPE=FUNCTION | NAME=lasx_xvfclass_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfclass_d(a: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfclass_d(transmute(a))) }
}
```

## Block 327
**Metadata**: AST_ID=327 | TYPE=FUNCTION | NAME=lasx_xvfsqrt_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfsqrt_s(a: m256) -> m256 {
    unsafe { transmute(__lasx_xvfsqrt_s(transmute(a))) }
}
```

## Block 328
**Metadata**: AST_ID=328 | TYPE=FUNCTION | NAME=lasx_xvfsqrt_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfsqrt_d(a: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfsqrt_d(transmute(a))) }
}
```

## Block 329
**Metadata**: AST_ID=329 | TYPE=FUNCTION | NAME=lasx_xvfrecip_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrecip_s(a: m256) -> m256 {
    unsafe { transmute(__lasx_xvfrecip_s(transmute(a))) }
}
```

## Block 330
**Metadata**: AST_ID=330 | TYPE=FUNCTION | NAME=lasx_xvfrecip_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrecip_d(a: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfrecip_d(transmute(a))) }
}
```

## Block 331
**Metadata**: AST_ID=331 | TYPE=FUNCTION | NAME=lasx_xvfrecipe_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx,frecipe")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrecipe_s(a: m256) -> m256 {
    unsafe { transmute(__lasx_xvfrecipe_s(transmute(a))) }
}
```

## Block 332
**Metadata**: AST_ID=332 | TYPE=FUNCTION | NAME=lasx_xvfrecipe_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx,frecipe")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrecipe_d(a: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfrecipe_d(transmute(a))) }
}
```

## Block 333
**Metadata**: AST_ID=333 | TYPE=FUNCTION | NAME=lasx_xvfrsqrte_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx,frecipe")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrsqrte_s(a: m256) -> m256 {
    unsafe { transmute(__lasx_xvfrsqrte_s(transmute(a))) }
}
```

## Block 334
**Metadata**: AST_ID=334 | TYPE=FUNCTION | NAME=lasx_xvfrsqrte_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx,frecipe")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrsqrte_d(a: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfrsqrte_d(transmute(a))) }
}
```

## Block 335
**Metadata**: AST_ID=335 | TYPE=FUNCTION | NAME=lasx_xvfrint_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrint_s(a: m256) -> m256 {
    unsafe { transmute(__lasx_xvfrint_s(transmute(a))) }
}
```

## Block 336
**Metadata**: AST_ID=336 | TYPE=FUNCTION | NAME=lasx_xvfrint_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrint_d(a: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfrint_d(transmute(a))) }
}
```

## Block 337
**Metadata**: AST_ID=337 | TYPE=FUNCTION | NAME=lasx_xvfrsqrt_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrsqrt_s(a: m256) -> m256 {
    unsafe { transmute(__lasx_xvfrsqrt_s(transmute(a))) }
}
```

## Block 338
**Metadata**: AST_ID=338 | TYPE=FUNCTION | NAME=lasx_xvfrsqrt_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrsqrt_d(a: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfrsqrt_d(transmute(a))) }
}
```

## Block 339
**Metadata**: AST_ID=339 | TYPE=FUNCTION | NAME=lasx_xvflogb_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvflogb_s(a: m256) -> m256 {
    unsafe { transmute(__lasx_xvflogb_s(transmute(a))) }
}
```

## Block 340
**Metadata**: AST_ID=340 | TYPE=FUNCTION | NAME=lasx_xvflogb_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvflogb_d(a: m256d) -> m256d {
    unsafe { transmute(__lasx_xvflogb_d(transmute(a))) }
}
```

## Block 341
**Metadata**: AST_ID=341 | TYPE=FUNCTION | NAME=lasx_xvfcvth_s_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcvth_s_h(a: m256i) -> m256 {
    unsafe { transmute(__lasx_xvfcvth_s_h(transmute(a))) }
}
```

## Block 342
**Metadata**: AST_ID=342 | TYPE=FUNCTION | NAME=lasx_xvfcvth_d_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcvth_d_s(a: m256) -> m256d {
    unsafe { transmute(__lasx_xvfcvth_d_s(transmute(a))) }
}
```

## Block 343
**Metadata**: AST_ID=343 | TYPE=FUNCTION | NAME=lasx_xvfcvtl_s_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcvtl_s_h(a: m256i) -> m256 {
    unsafe { transmute(__lasx_xvfcvtl_s_h(transmute(a))) }
}
```

## Block 344
**Metadata**: AST_ID=344 | TYPE=FUNCTION | NAME=lasx_xvfcvtl_d_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcvtl_d_s(a: m256) -> m256d {
    unsafe { transmute(__lasx_xvfcvtl_d_s(transmute(a))) }
}
```

## Block 345
**Metadata**: AST_ID=345 | TYPE=FUNCTION | NAME=lasx_xvftint_w_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftint_w_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftint_w_s(transmute(a))) }
}
```

## Block 346
**Metadata**: AST_ID=346 | TYPE=FUNCTION | NAME=lasx_xvftint_l_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftint_l_d(a: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftint_l_d(transmute(a))) }
}
```

## Block 347
**Metadata**: AST_ID=347 | TYPE=FUNCTION | NAME=lasx_xvftint_wu_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftint_wu_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftint_wu_s(transmute(a))) }
}
```

## Block 348
**Metadata**: AST_ID=348 | TYPE=FUNCTION | NAME=lasx_xvftint_lu_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftint_lu_d(a: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftint_lu_d(transmute(a))) }
}
```

## Block 349
**Metadata**: AST_ID=349 | TYPE=FUNCTION | NAME=lasx_xvftintrz_w_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrz_w_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrz_w_s(transmute(a))) }
}
```

## Block 350
**Metadata**: AST_ID=350 | TYPE=FUNCTION | NAME=lasx_xvftintrz_l_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrz_l_d(a: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftintrz_l_d(transmute(a))) }
}
```

## Block 351
**Metadata**: AST_ID=351 | TYPE=FUNCTION | NAME=lasx_xvftintrz_wu_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrz_wu_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrz_wu_s(transmute(a))) }
}
```

## Block 352
**Metadata**: AST_ID=352 | TYPE=FUNCTION | NAME=lasx_xvftintrz_lu_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrz_lu_d(a: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftintrz_lu_d(transmute(a))) }
}
```

## Block 353
**Metadata**: AST_ID=353 | TYPE=FUNCTION | NAME=lasx_xvffint_s_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvffint_s_w(a: m256i) -> m256 {
    unsafe { transmute(__lasx_xvffint_s_w(transmute(a))) }
}
```

## Block 354
**Metadata**: AST_ID=354 | TYPE=FUNCTION | NAME=lasx_xvffint_d_l | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvffint_d_l(a: m256i) -> m256d {
    unsafe { transmute(__lasx_xvffint_d_l(transmute(a))) }
}
```

## Block 355
**Metadata**: AST_ID=355 | TYPE=FUNCTION | NAME=lasx_xvffint_s_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvffint_s_wu(a: m256i) -> m256 {
    unsafe { transmute(__lasx_xvffint_s_wu(transmute(a))) }
}
```

## Block 356
**Metadata**: AST_ID=356 | TYPE=FUNCTION | NAME=lasx_xvffint_d_lu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvffint_d_lu(a: m256i) -> m256d {
    unsafe { transmute(__lasx_xvffint_d_lu(transmute(a))) }
}
```

## Block 357
**Metadata**: AST_ID=357 | TYPE=FUNCTION | NAME=lasx_xvreplve_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplve_b(a: m256i, b: i32) -> m256i {
    unsafe { transmute(__lasx_xvreplve_b(transmute(a), transmute(b))) }
}
```

## Block 358
**Metadata**: AST_ID=358 | TYPE=FUNCTION | NAME=lasx_xvreplve_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplve_h(a: m256i, b: i32) -> m256i {
    unsafe { transmute(__lasx_xvreplve_h(transmute(a), transmute(b))) }
}
```

## Block 359
**Metadata**: AST_ID=359 | TYPE=FUNCTION | NAME=lasx_xvreplve_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplve_w(a: m256i, b: i32) -> m256i {
    unsafe { transmute(__lasx_xvreplve_w(transmute(a), transmute(b))) }
}
```

## Block 360
**Metadata**: AST_ID=360 | TYPE=FUNCTION | NAME=lasx_xvreplve_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplve_d(a: m256i, b: i32) -> m256i {
    unsafe { transmute(__lasx_xvreplve_d(transmute(a), transmute(b))) }
}
```

## Block 361
**Metadata**: AST_ID=361 | TYPE=FUNCTION | NAME=lasx_xvpermi_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpermi_w<const IMM8: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvpermi_w(transmute(a), transmute(b), IMM8)) }
}
```

## Block 362
**Metadata**: AST_ID=362 | TYPE=FUNCTION | NAME=lasx_xvandn_v | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvandn_v(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvandn_v(transmute(a), transmute(b))) }
}
```

## Block 363
**Metadata**: AST_ID=363 | TYPE=FUNCTION | NAME=lasx_xvneg_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvneg_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvneg_b(transmute(a))) }
}
```

## Block 364
**Metadata**: AST_ID=364 | TYPE=FUNCTION | NAME=lasx_xvneg_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvneg_h(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvneg_h(transmute(a))) }
}
```

## Block 365
**Metadata**: AST_ID=365 | TYPE=FUNCTION | NAME=lasx_xvneg_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvneg_w(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvneg_w(transmute(a))) }
}
```

## Block 366
**Metadata**: AST_ID=366 | TYPE=FUNCTION | NAME=lasx_xvneg_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvneg_d(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvneg_d(transmute(a))) }
}
```

## Block 367
**Metadata**: AST_ID=367 | TYPE=FUNCTION | NAME=lasx_xvmuh_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmuh_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmuh_b(transmute(a), transmute(b))) }
}
```

## Block 368
**Metadata**: AST_ID=368 | TYPE=FUNCTION | NAME=lasx_xvmuh_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmuh_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmuh_h(transmute(a), transmute(b))) }
}
```

## Block 369
**Metadata**: AST_ID=369 | TYPE=FUNCTION | NAME=lasx_xvmuh_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmuh_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmuh_w(transmute(a), transmute(b))) }
}
```

## Block 370
**Metadata**: AST_ID=370 | TYPE=FUNCTION | NAME=lasx_xvmuh_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmuh_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmuh_d(transmute(a), transmute(b))) }
}
```

## Block 371
**Metadata**: AST_ID=371 | TYPE=FUNCTION | NAME=lasx_xvmuh_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmuh_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmuh_bu(transmute(a), transmute(b))) }
}
```

## Block 372
**Metadata**: AST_ID=372 | TYPE=FUNCTION | NAME=lasx_xvmuh_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmuh_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmuh_hu(transmute(a), transmute(b))) }
}
```

## Block 373
**Metadata**: AST_ID=373 | TYPE=FUNCTION | NAME=lasx_xvmuh_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmuh_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmuh_wu(transmute(a), transmute(b))) }
}
```

## Block 374
**Metadata**: AST_ID=374 | TYPE=FUNCTION | NAME=lasx_xvmuh_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmuh_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmuh_du(transmute(a), transmute(b))) }
}
```

## Block 375
**Metadata**: AST_ID=375 | TYPE=FUNCTION | NAME=lasx_xvsllwil_h_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsllwil_h_b<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvsllwil_h_b(transmute(a), IMM3)) }
}
```

## Block 376
**Metadata**: AST_ID=376 | TYPE=FUNCTION | NAME=lasx_xvsllwil_w_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsllwil_w_h<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsllwil_w_h(transmute(a), IMM4)) }
}
```

## Block 377
**Metadata**: AST_ID=377 | TYPE=FUNCTION | NAME=lasx_xvsllwil_d_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsllwil_d_w<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsllwil_d_w(transmute(a), IMM5)) }
}
```

## Block 378
**Metadata**: AST_ID=378 | TYPE=FUNCTION | NAME=lasx_xvsllwil_hu_bu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsllwil_hu_bu<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvsllwil_hu_bu(transmute(a), IMM3)) }
}
```

## Block 379
**Metadata**: AST_ID=379 | TYPE=FUNCTION | NAME=lasx_xvsllwil_wu_hu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsllwil_wu_hu<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsllwil_wu_hu(transmute(a), IMM4)) }
}
```

## Block 380
**Metadata**: AST_ID=380 | TYPE=FUNCTION | NAME=lasx_xvsllwil_du_wu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsllwil_du_wu<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsllwil_du_wu(transmute(a), IMM5)) }
}
```

## Block 381
**Metadata**: AST_ID=381 | TYPE=FUNCTION | NAME=lasx_xvsran_b_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsran_b_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsran_b_h(transmute(a), transmute(b))) }
}
```

## Block 382
**Metadata**: AST_ID=382 | TYPE=FUNCTION | NAME=lasx_xvsran_h_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsran_h_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsran_h_w(transmute(a), transmute(b))) }
}
```

## Block 383
**Metadata**: AST_ID=383 | TYPE=FUNCTION | NAME=lasx_xvsran_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsran_w_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsran_w_d(transmute(a), transmute(b))) }
}
```

## Block 384
**Metadata**: AST_ID=384 | TYPE=FUNCTION | NAME=lasx_xvssran_b_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssran_b_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssran_b_h(transmute(a), transmute(b))) }
}
```

## Block 385
**Metadata**: AST_ID=385 | TYPE=FUNCTION | NAME=lasx_xvssran_h_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssran_h_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssran_h_w(transmute(a), transmute(b))) }
}
```

## Block 386
**Metadata**: AST_ID=386 | TYPE=FUNCTION | NAME=lasx_xvssran_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssran_w_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssran_w_d(transmute(a), transmute(b))) }
}
```

## Block 387
**Metadata**: AST_ID=387 | TYPE=FUNCTION | NAME=lasx_xvssran_bu_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssran_bu_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssran_bu_h(transmute(a), transmute(b))) }
}
```

## Block 388
**Metadata**: AST_ID=388 | TYPE=FUNCTION | NAME=lasx_xvssran_hu_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssran_hu_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssran_hu_w(transmute(a), transmute(b))) }
}
```

## Block 389
**Metadata**: AST_ID=389 | TYPE=FUNCTION | NAME=lasx_xvssran_wu_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssran_wu_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssran_wu_d(transmute(a), transmute(b))) }
}
```

## Block 390
**Metadata**: AST_ID=390 | TYPE=FUNCTION | NAME=lasx_xvsrarn_b_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrarn_b_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrarn_b_h(transmute(a), transmute(b))) }
}
```

## Block 391
**Metadata**: AST_ID=391 | TYPE=FUNCTION | NAME=lasx_xvsrarn_h_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrarn_h_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrarn_h_w(transmute(a), transmute(b))) }
}
```

## Block 392
**Metadata**: AST_ID=392 | TYPE=FUNCTION | NAME=lasx_xvsrarn_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrarn_w_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrarn_w_d(transmute(a), transmute(b))) }
}
```

## Block 393
**Metadata**: AST_ID=393 | TYPE=FUNCTION | NAME=lasx_xvssrarn_b_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarn_b_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrarn_b_h(transmute(a), transmute(b))) }
}
```

## Block 394
**Metadata**: AST_ID=394 | TYPE=FUNCTION | NAME=lasx_xvssrarn_h_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarn_h_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrarn_h_w(transmute(a), transmute(b))) }
}
```

## Block 395
**Metadata**: AST_ID=395 | TYPE=FUNCTION | NAME=lasx_xvssrarn_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarn_w_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrarn_w_d(transmute(a), transmute(b))) }
}
```

## Block 396
**Metadata**: AST_ID=396 | TYPE=FUNCTION | NAME=lasx_xvssrarn_bu_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarn_bu_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrarn_bu_h(transmute(a), transmute(b))) }
}
```

## Block 397
**Metadata**: AST_ID=397 | TYPE=FUNCTION | NAME=lasx_xvssrarn_hu_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarn_hu_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrarn_hu_w(transmute(a), transmute(b))) }
}
```

## Block 398
**Metadata**: AST_ID=398 | TYPE=FUNCTION | NAME=lasx_xvssrarn_wu_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarn_wu_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrarn_wu_d(transmute(a), transmute(b))) }
}
```

## Block 399
**Metadata**: AST_ID=399 | TYPE=FUNCTION | NAME=lasx_xvsrln_b_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrln_b_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrln_b_h(transmute(a), transmute(b))) }
}
```

## Block 400
**Metadata**: AST_ID=400 | TYPE=FUNCTION | NAME=lasx_xvsrln_h_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrln_h_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrln_h_w(transmute(a), transmute(b))) }
}
```

## Block 401
**Metadata**: AST_ID=401 | TYPE=FUNCTION | NAME=lasx_xvsrln_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrln_w_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrln_w_d(transmute(a), transmute(b))) }
}
```

## Block 402
**Metadata**: AST_ID=402 | TYPE=FUNCTION | NAME=lasx_xvssrln_bu_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrln_bu_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrln_bu_h(transmute(a), transmute(b))) }
}
```

## Block 403
**Metadata**: AST_ID=403 | TYPE=FUNCTION | NAME=lasx_xvssrln_hu_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrln_hu_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrln_hu_w(transmute(a), transmute(b))) }
}
```

## Block 404
**Metadata**: AST_ID=404 | TYPE=FUNCTION | NAME=lasx_xvssrln_wu_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrln_wu_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrln_wu_d(transmute(a), transmute(b))) }
}
```

## Block 405
**Metadata**: AST_ID=405 | TYPE=FUNCTION | NAME=lasx_xvsrlrn_b_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlrn_b_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrlrn_b_h(transmute(a), transmute(b))) }
}
```

## Block 406
**Metadata**: AST_ID=406 | TYPE=FUNCTION | NAME=lasx_xvsrlrn_h_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlrn_h_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrlrn_h_w(transmute(a), transmute(b))) }
}
```

## Block 407
**Metadata**: AST_ID=407 | TYPE=FUNCTION | NAME=lasx_xvsrlrn_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlrn_w_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsrlrn_w_d(transmute(a), transmute(b))) }
}
```

## Block 408
**Metadata**: AST_ID=408 | TYPE=FUNCTION | NAME=lasx_xvssrlrn_bu_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrn_bu_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrlrn_bu_h(transmute(a), transmute(b))) }
}
```

## Block 409
**Metadata**: AST_ID=409 | TYPE=FUNCTION | NAME=lasx_xvssrlrn_hu_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrn_hu_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrlrn_hu_w(transmute(a), transmute(b))) }
}
```

## Block 410
**Metadata**: AST_ID=410 | TYPE=FUNCTION | NAME=lasx_xvssrlrn_wu_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrn_wu_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrlrn_wu_d(transmute(a), transmute(b))) }
}
```

## Block 411
**Metadata**: AST_ID=411 | TYPE=FUNCTION | NAME=lasx_xvfrstpi_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrstpi_b<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvfrstpi_b(transmute(a), transmute(b), IMM5)) }
}
```

## Block 412
**Metadata**: AST_ID=412 | TYPE=FUNCTION | NAME=lasx_xvfrstpi_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrstpi_h<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvfrstpi_h(transmute(a), transmute(b), IMM5)) }
}
```

## Block 413
**Metadata**: AST_ID=413 | TYPE=FUNCTION | NAME=lasx_xvfrstp_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrstp_b(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvfrstp_b(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 414
**Metadata**: AST_ID=414 | TYPE=FUNCTION | NAME=lasx_xvfrstp_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrstp_h(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvfrstp_h(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 415
**Metadata**: AST_ID=415 | TYPE=FUNCTION | NAME=lasx_xvshuf4i_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvshuf4i_d<const IMM8: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvshuf4i_d(transmute(a), transmute(b), IMM8)) }
}
```

## Block 416
**Metadata**: AST_ID=416 | TYPE=FUNCTION | NAME=lasx_xvbsrl_v | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbsrl_v<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvbsrl_v(transmute(a), IMM5)) }
}
```

## Block 417
**Metadata**: AST_ID=417 | TYPE=FUNCTION | NAME=lasx_xvbsll_v | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvbsll_v<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvbsll_v(transmute(a), IMM5)) }
}
```

## Block 418
**Metadata**: AST_ID=418 | TYPE=FUNCTION | NAME=lasx_xvextrins_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvextrins_b<const IMM8: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvextrins_b(transmute(a), transmute(b), IMM8)) }
}
```

## Block 419
**Metadata**: AST_ID=419 | TYPE=FUNCTION | NAME=lasx_xvextrins_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvextrins_h<const IMM8: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvextrins_h(transmute(a), transmute(b), IMM8)) }
}
```

## Block 420
**Metadata**: AST_ID=420 | TYPE=FUNCTION | NAME=lasx_xvextrins_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvextrins_w<const IMM8: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvextrins_w(transmute(a), transmute(b), IMM8)) }
}
```

## Block 421
**Metadata**: AST_ID=421 | TYPE=FUNCTION | NAME=lasx_xvextrins_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvextrins_d<const IMM8: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvextrins_d(transmute(a), transmute(b), IMM8)) }
}
```

## Block 422
**Metadata**: AST_ID=422 | TYPE=FUNCTION | NAME=lasx_xvmskltz_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmskltz_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmskltz_b(transmute(a))) }
}
```

## Block 423
**Metadata**: AST_ID=423 | TYPE=FUNCTION | NAME=lasx_xvmskltz_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmskltz_h(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmskltz_h(transmute(a))) }
}
```

## Block 424
**Metadata**: AST_ID=424 | TYPE=FUNCTION | NAME=lasx_xvmskltz_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmskltz_w(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmskltz_w(transmute(a))) }
}
```

## Block 425
**Metadata**: AST_ID=425 | TYPE=FUNCTION | NAME=lasx_xvmskltz_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmskltz_d(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmskltz_d(transmute(a))) }
}
```

## Block 426
**Metadata**: AST_ID=426 | TYPE=FUNCTION | NAME=lasx_xvsigncov_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsigncov_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsigncov_b(transmute(a), transmute(b))) }
}
```

## Block 427
**Metadata**: AST_ID=427 | TYPE=FUNCTION | NAME=lasx_xvsigncov_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsigncov_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsigncov_h(transmute(a), transmute(b))) }
}
```

## Block 428
**Metadata**: AST_ID=428 | TYPE=FUNCTION | NAME=lasx_xvsigncov_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsigncov_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsigncov_w(transmute(a), transmute(b))) }
}
```

## Block 429
**Metadata**: AST_ID=429 | TYPE=FUNCTION | NAME=lasx_xvsigncov_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsigncov_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsigncov_d(transmute(a), transmute(b))) }
}
```

## Block 430
**Metadata**: AST_ID=430 | TYPE=FUNCTION | NAME=lasx_xvfmadd_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmadd_s(a: m256, b: m256, c: m256) -> m256 {
    unsafe { transmute(__lasx_xvfmadd_s(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 431
**Metadata**: AST_ID=431 | TYPE=FUNCTION | NAME=lasx_xvfmadd_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmadd_d(a: m256d, b: m256d, c: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfmadd_d(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 432
**Metadata**: AST_ID=432 | TYPE=FUNCTION | NAME=lasx_xvfmsub_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmsub_s(a: m256, b: m256, c: m256) -> m256 {
    unsafe { transmute(__lasx_xvfmsub_s(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 433
**Metadata**: AST_ID=433 | TYPE=FUNCTION | NAME=lasx_xvfmsub_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfmsub_d(a: m256d, b: m256d, c: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfmsub_d(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 434
**Metadata**: AST_ID=434 | TYPE=FUNCTION | NAME=lasx_xvfnmadd_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfnmadd_s(a: m256, b: m256, c: m256) -> m256 {
    unsafe { transmute(__lasx_xvfnmadd_s(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 435
**Metadata**: AST_ID=435 | TYPE=FUNCTION | NAME=lasx_xvfnmadd_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfnmadd_d(a: m256d, b: m256d, c: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfnmadd_d(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 436
**Metadata**: AST_ID=436 | TYPE=FUNCTION | NAME=lasx_xvfnmsub_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfnmsub_s(a: m256, b: m256, c: m256) -> m256 {
    unsafe { transmute(__lasx_xvfnmsub_s(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 437
**Metadata**: AST_ID=437 | TYPE=FUNCTION | NAME=lasx_xvfnmsub_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfnmsub_d(a: m256d, b: m256d, c: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfnmsub_d(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 438
**Metadata**: AST_ID=438 | TYPE=FUNCTION | NAME=lasx_xvftintrne_w_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrne_w_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrne_w_s(transmute(a))) }
}
```

## Block 439
**Metadata**: AST_ID=439 | TYPE=FUNCTION | NAME=lasx_xvftintrne_l_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrne_l_d(a: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftintrne_l_d(transmute(a))) }
}
```

## Block 440
**Metadata**: AST_ID=440 | TYPE=FUNCTION | NAME=lasx_xvftintrp_w_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrp_w_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrp_w_s(transmute(a))) }
}
```

## Block 441
**Metadata**: AST_ID=441 | TYPE=FUNCTION | NAME=lasx_xvftintrp_l_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrp_l_d(a: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftintrp_l_d(transmute(a))) }
}
```

## Block 442
**Metadata**: AST_ID=442 | TYPE=FUNCTION | NAME=lasx_xvftintrm_w_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrm_w_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrm_w_s(transmute(a))) }
}
```

## Block 443
**Metadata**: AST_ID=443 | TYPE=FUNCTION | NAME=lasx_xvftintrm_l_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrm_l_d(a: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftintrm_l_d(transmute(a))) }
}
```

## Block 444
**Metadata**: AST_ID=444 | TYPE=FUNCTION | NAME=lasx_xvftint_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftint_w_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftint_w_d(transmute(a), transmute(b))) }
}
```

## Block 445
**Metadata**: AST_ID=445 | TYPE=FUNCTION | NAME=lasx_xvffint_s_l | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvffint_s_l(a: m256i, b: m256i) -> m256 {
    unsafe { transmute(__lasx_xvffint_s_l(transmute(a), transmute(b))) }
}
```

## Block 446
**Metadata**: AST_ID=446 | TYPE=FUNCTION | NAME=lasx_xvftintrz_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrz_w_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftintrz_w_d(transmute(a), transmute(b))) }
}
```

## Block 447
**Metadata**: AST_ID=447 | TYPE=FUNCTION | NAME=lasx_xvftintrp_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrp_w_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftintrp_w_d(transmute(a), transmute(b))) }
}
```

## Block 448
**Metadata**: AST_ID=448 | TYPE=FUNCTION | NAME=lasx_xvftintrm_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrm_w_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftintrm_w_d(transmute(a), transmute(b))) }
}
```

## Block 449
**Metadata**: AST_ID=449 | TYPE=FUNCTION | NAME=lasx_xvftintrne_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrne_w_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvftintrne_w_d(transmute(a), transmute(b))) }
}
```

## Block 450
**Metadata**: AST_ID=450 | TYPE=FUNCTION | NAME=lasx_xvftinth_l_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftinth_l_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftinth_l_s(transmute(a))) }
}
```

## Block 451
**Metadata**: AST_ID=451 | TYPE=FUNCTION | NAME=lasx_xvftintl_l_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintl_l_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintl_l_s(transmute(a))) }
}
```

## Block 452
**Metadata**: AST_ID=452 | TYPE=FUNCTION | NAME=lasx_xvffinth_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvffinth_d_w(a: m256i) -> m256d {
    unsafe { transmute(__lasx_xvffinth_d_w(transmute(a))) }
}
```

## Block 453
**Metadata**: AST_ID=453 | TYPE=FUNCTION | NAME=lasx_xvffintl_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvffintl_d_w(a: m256i) -> m256d {
    unsafe { transmute(__lasx_xvffintl_d_w(transmute(a))) }
}
```

## Block 454
**Metadata**: AST_ID=454 | TYPE=FUNCTION | NAME=lasx_xvftintrzh_l_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrzh_l_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrzh_l_s(transmute(a))) }
}
```

## Block 455
**Metadata**: AST_ID=455 | TYPE=FUNCTION | NAME=lasx_xvftintrzl_l_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrzl_l_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrzl_l_s(transmute(a))) }
}
```

## Block 456
**Metadata**: AST_ID=456 | TYPE=FUNCTION | NAME=lasx_xvftintrph_l_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrph_l_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrph_l_s(transmute(a))) }
}
```

## Block 457
**Metadata**: AST_ID=457 | TYPE=FUNCTION | NAME=lasx_xvftintrpl_l_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrpl_l_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrpl_l_s(transmute(a))) }
}
```

## Block 458
**Metadata**: AST_ID=458 | TYPE=FUNCTION | NAME=lasx_xvftintrmh_l_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrmh_l_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrmh_l_s(transmute(a))) }
}
```

## Block 459
**Metadata**: AST_ID=459 | TYPE=FUNCTION | NAME=lasx_xvftintrml_l_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrml_l_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrml_l_s(transmute(a))) }
}
```

## Block 460
**Metadata**: AST_ID=460 | TYPE=FUNCTION | NAME=lasx_xvftintrneh_l_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrneh_l_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrneh_l_s(transmute(a))) }
}
```

## Block 461
**Metadata**: AST_ID=461 | TYPE=FUNCTION | NAME=lasx_xvftintrnel_l_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvftintrnel_l_s(a: m256) -> m256i {
    unsafe { transmute(__lasx_xvftintrnel_l_s(transmute(a))) }
}
```

## Block 462
**Metadata**: AST_ID=462 | TYPE=FUNCTION | NAME=lasx_xvfrintrne_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrintrne_s(a: m256) -> m256 {
    unsafe { transmute(__lasx_xvfrintrne_s(transmute(a))) }
}
```

## Block 463
**Metadata**: AST_ID=463 | TYPE=FUNCTION | NAME=lasx_xvfrintrne_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrintrne_d(a: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfrintrne_d(transmute(a))) }
}
```

## Block 464
**Metadata**: AST_ID=464 | TYPE=FUNCTION | NAME=lasx_xvfrintrz_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrintrz_s(a: m256) -> m256 {
    unsafe { transmute(__lasx_xvfrintrz_s(transmute(a))) }
}
```

## Block 465
**Metadata**: AST_ID=465 | TYPE=FUNCTION | NAME=lasx_xvfrintrz_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrintrz_d(a: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfrintrz_d(transmute(a))) }
}
```

## Block 466
**Metadata**: AST_ID=466 | TYPE=FUNCTION | NAME=lasx_xvfrintrp_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrintrp_s(a: m256) -> m256 {
    unsafe { transmute(__lasx_xvfrintrp_s(transmute(a))) }
}
```

## Block 467
**Metadata**: AST_ID=467 | TYPE=FUNCTION | NAME=lasx_xvfrintrp_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrintrp_d(a: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfrintrp_d(transmute(a))) }
}
```

## Block 468
**Metadata**: AST_ID=468 | TYPE=FUNCTION | NAME=lasx_xvfrintrm_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrintrm_s(a: m256) -> m256 {
    unsafe { transmute(__lasx_xvfrintrm_s(transmute(a))) }
}
```

## Block 469
**Metadata**: AST_ID=469 | TYPE=FUNCTION | NAME=lasx_xvfrintrm_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfrintrm_d(a: m256d) -> m256d {
    unsafe { transmute(__lasx_xvfrintrm_d(transmute(a))) }
}
```

## Block 470
**Metadata**: AST_ID=470 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvld<const IMM_S12: i32>(mem_addr: *const i8) -> m256i {
    static_assert_simm_bits!(IMM_S12, 12);
    transmute(__lasx_xvld(mem_addr, IMM_S12))
}
```

## Block 471
**Metadata**: AST_ID=471 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvst<const IMM_S12: i32>(a: m256i, mem_addr: *mut i8) {
    static_assert_simm_bits!(IMM_S12, 12);
    transmute(__lasx_xvst(transmute(a), mem_addr, IMM_S12))
}
```

## Block 472
**Metadata**: AST_ID=472 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=10

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvstelm_b<const IMM_S8: i32, const IMM4: u32>(a: m256i, mem_addr: *mut i8) {
    static_assert_simm_bits!(IMM_S8, 8);
    static_assert_uimm_bits!(IMM4, 4);
    transmute(__lasx_xvstelm_b(transmute(a), mem_addr, IMM_S8, IMM4))
}
```

## Block 473
**Metadata**: AST_ID=473 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=10

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvstelm_h<const IMM_S8: i32, const IMM3: u32>(a: m256i, mem_addr: *mut i8) {
    static_assert_simm_bits!(IMM_S8, 8);
    static_assert_uimm_bits!(IMM3, 3);
    transmute(__lasx_xvstelm_h(transmute(a), mem_addr, IMM_S8, IMM3))
}
```

## Block 474
**Metadata**: AST_ID=474 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=10

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvstelm_w<const IMM_S8: i32, const IMM2: u32>(a: m256i, mem_addr: *mut i8) {
    static_assert_simm_bits!(IMM_S8, 8);
    static_assert_uimm_bits!(IMM2, 2);
    transmute(__lasx_xvstelm_w(transmute(a), mem_addr, IMM_S8, IMM2))
}
```

## Block 475
**Metadata**: AST_ID=475 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=10

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvstelm_d<const IMM_S8: i32, const IMM1: u32>(a: m256i, mem_addr: *mut i8) {
    static_assert_simm_bits!(IMM_S8, 8);
    static_assert_uimm_bits!(IMM1, 1);
    transmute(__lasx_xvstelm_d(transmute(a), mem_addr, IMM_S8, IMM1))
}
```

## Block 476
**Metadata**: AST_ID=476 | TYPE=FUNCTION | NAME=lasx_xvinsve0_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvinsve0_w<const IMM3: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvinsve0_w(transmute(a), transmute(b), IMM3)) }
}
```

## Block 477
**Metadata**: AST_ID=477 | TYPE=FUNCTION | NAME=lasx_xvinsve0_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvinsve0_d<const IMM2: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM2, 2);
    unsafe { transmute(__lasx_xvinsve0_d(transmute(a), transmute(b), IMM2)) }
}
```

## Block 478
**Metadata**: AST_ID=478 | TYPE=FUNCTION | NAME=lasx_xvpickve_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickve_w<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvpickve_w(transmute(a), IMM3)) }
}
```

## Block 479
**Metadata**: AST_ID=479 | TYPE=FUNCTION | NAME=lasx_xvpickve_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickve_d<const IMM2: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM2, 2);
    unsafe { transmute(__lasx_xvpickve_d(transmute(a), IMM2)) }
}
```

## Block 480
**Metadata**: AST_ID=480 | TYPE=FUNCTION | NAME=lasx_xvssrlrn_b_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrn_b_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrlrn_b_h(transmute(a), transmute(b))) }
}
```

## Block 481
**Metadata**: AST_ID=481 | TYPE=FUNCTION | NAME=lasx_xvssrlrn_h_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrn_h_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrlrn_h_w(transmute(a), transmute(b))) }
}
```

## Block 482
**Metadata**: AST_ID=482 | TYPE=FUNCTION | NAME=lasx_xvssrlrn_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrn_w_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrlrn_w_d(transmute(a), transmute(b))) }
}
```

## Block 483
**Metadata**: AST_ID=483 | TYPE=FUNCTION | NAME=lasx_xvssrln_b_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrln_b_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrln_b_h(transmute(a), transmute(b))) }
}
```

## Block 484
**Metadata**: AST_ID=484 | TYPE=FUNCTION | NAME=lasx_xvssrln_h_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrln_h_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrln_h_w(transmute(a), transmute(b))) }
}
```

## Block 485
**Metadata**: AST_ID=485 | TYPE=FUNCTION | NAME=lasx_xvssrln_w_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrln_w_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvssrln_w_d(transmute(a), transmute(b))) }
}
```

## Block 486
**Metadata**: AST_ID=486 | TYPE=FUNCTION | NAME=lasx_xvorn_v | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvorn_v(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvorn_v(transmute(a), transmute(b))) }
}
```

## Block 487
**Metadata**: AST_ID=487 | TYPE=FUNCTION | NAME=lasx_xvldi | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(0)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvldi<const IMM_S13: i32>() -> m256i {
    static_assert_simm_bits!(IMM_S13, 13);
    unsafe { transmute(__lasx_xvldi(IMM_S13)) }
}
```

## Block 488
**Metadata**: AST_ID=488 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvldx(mem_addr: *const i8, b: i64) -> m256i {
    transmute(__lasx_xvldx(mem_addr, transmute(b)))
}
```

## Block 489
**Metadata**: AST_ID=489 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvstx(a: m256i, mem_addr: *mut i8, b: i64) {
    transmute(__lasx_xvstx(transmute(a), mem_addr, transmute(b)))
}
```

## Block 490
**Metadata**: AST_ID=490 | TYPE=FUNCTION | NAME=lasx_xvextl_qu_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvextl_qu_du(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvextl_qu_du(transmute(a))) }
}
```

## Block 491
**Metadata**: AST_ID=491 | TYPE=FUNCTION | NAME=lasx_xvinsgr2vr_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvinsgr2vr_w<const IMM3: u32>(a: m256i, b: i32) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvinsgr2vr_w(transmute(a), transmute(b), IMM3)) }
}
```

## Block 492
**Metadata**: AST_ID=492 | TYPE=FUNCTION | NAME=lasx_xvinsgr2vr_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvinsgr2vr_d<const IMM2: u32>(a: m256i, b: i64) -> m256i {
    static_assert_uimm_bits!(IMM2, 2);
    unsafe { transmute(__lasx_xvinsgr2vr_d(transmute(a), transmute(b), IMM2)) }
}
```

## Block 493
**Metadata**: AST_ID=493 | TYPE=FUNCTION | NAME=lasx_xvreplve0_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplve0_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvreplve0_b(transmute(a))) }
}
```

## Block 494
**Metadata**: AST_ID=494 | TYPE=FUNCTION | NAME=lasx_xvreplve0_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplve0_h(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvreplve0_h(transmute(a))) }
}
```

## Block 495
**Metadata**: AST_ID=495 | TYPE=FUNCTION | NAME=lasx_xvreplve0_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplve0_w(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvreplve0_w(transmute(a))) }
}
```

## Block 496
**Metadata**: AST_ID=496 | TYPE=FUNCTION | NAME=lasx_xvreplve0_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplve0_d(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvreplve0_d(transmute(a))) }
}
```

## Block 497
**Metadata**: AST_ID=497 | TYPE=FUNCTION | NAME=lasx_xvreplve0_q | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvreplve0_q(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvreplve0_q(transmute(a))) }
}
```

## Block 498
**Metadata**: AST_ID=498 | TYPE=FUNCTION | NAME=lasx_vext2xv_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_h_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_h_b(transmute(a))) }
}
```

## Block 499
**Metadata**: AST_ID=499 | TYPE=FUNCTION | NAME=lasx_vext2xv_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_w_h(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_w_h(transmute(a))) }
}
```

## Block 500
**Metadata**: AST_ID=500 | TYPE=FUNCTION | NAME=lasx_vext2xv_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_d_w(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_d_w(transmute(a))) }
}
```

## Block 501
**Metadata**: AST_ID=501 | TYPE=FUNCTION | NAME=lasx_vext2xv_w_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_w_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_w_b(transmute(a))) }
}
```

## Block 502
**Metadata**: AST_ID=502 | TYPE=FUNCTION | NAME=lasx_vext2xv_d_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_d_h(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_d_h(transmute(a))) }
}
```

## Block 503
**Metadata**: AST_ID=503 | TYPE=FUNCTION | NAME=lasx_vext2xv_d_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_d_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_d_b(transmute(a))) }
}
```

## Block 504
**Metadata**: AST_ID=504 | TYPE=FUNCTION | NAME=lasx_vext2xv_hu_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_hu_bu(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_hu_bu(transmute(a))) }
}
```

## Block 505
**Metadata**: AST_ID=505 | TYPE=FUNCTION | NAME=lasx_vext2xv_wu_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_wu_hu(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_wu_hu(transmute(a))) }
}
```

## Block 506
**Metadata**: AST_ID=506 | TYPE=FUNCTION | NAME=lasx_vext2xv_du_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_du_wu(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_du_wu(transmute(a))) }
}
```

## Block 507
**Metadata**: AST_ID=507 | TYPE=FUNCTION | NAME=lasx_vext2xv_wu_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_wu_bu(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_wu_bu(transmute(a))) }
}
```

## Block 508
**Metadata**: AST_ID=508 | TYPE=FUNCTION | NAME=lasx_vext2xv_du_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_du_hu(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_du_hu(transmute(a))) }
}
```

## Block 509
**Metadata**: AST_ID=509 | TYPE=FUNCTION | NAME=lasx_vext2xv_du_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_vext2xv_du_bu(a: m256i) -> m256i {
    unsafe { transmute(__lasx_vext2xv_du_bu(transmute(a))) }
}
```

## Block 510
**Metadata**: AST_ID=510 | TYPE=FUNCTION | NAME=lasx_xvpermi_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpermi_q<const IMM8: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvpermi_q(transmute(a), transmute(b), IMM8)) }
}
```

## Block 511
**Metadata**: AST_ID=511 | TYPE=FUNCTION | NAME=lasx_xvpermi_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpermi_d<const IMM8: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM8, 8);
    unsafe { transmute(__lasx_xvpermi_d(transmute(a), IMM8)) }
}
```

## Block 512
**Metadata**: AST_ID=512 | TYPE=FUNCTION | NAME=lasx_xvperm_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvperm_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvperm_w(transmute(a), transmute(b))) }
}
```

## Block 513
**Metadata**: AST_ID=513 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvldrepl_b<const IMM_S12: i32>(mem_addr: *const i8) -> m256i {
    static_assert_simm_bits!(IMM_S12, 12);
    transmute(__lasx_xvldrepl_b(mem_addr, IMM_S12))
}
```

## Block 514
**Metadata**: AST_ID=514 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvldrepl_h<const IMM_S11: i32>(mem_addr: *const i8) -> m256i {
    static_assert_simm_bits!(IMM_S11, 11);
    transmute(__lasx_xvldrepl_h(mem_addr, IMM_S11))
}
```

## Block 515
**Metadata**: AST_ID=515 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvldrepl_w<const IMM_S10: i32>(mem_addr: *const i8) -> m256i {
    static_assert_simm_bits!(IMM_S10, 10);
    transmute(__lasx_xvldrepl_w(mem_addr, IMM_S10))
}
```

## Block 516
**Metadata**: AST_ID=516 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub unsafe fn lasx_xvldrepl_d<const IMM_S9: i32>(mem_addr: *const i8) -> m256i {
    static_assert_simm_bits!(IMM_S9, 9);
    transmute(__lasx_xvldrepl_d(mem_addr, IMM_S9))
}
```

## Block 517
**Metadata**: AST_ID=517 | TYPE=FUNCTION | NAME=lasx_xvpickve2gr_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickve2gr_w<const IMM3: u32>(a: m256i) -> i32 {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvpickve2gr_w(transmute(a), IMM3)) }
}
```

## Block 518
**Metadata**: AST_ID=518 | TYPE=FUNCTION | NAME=lasx_xvpickve2gr_wu | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickve2gr_wu<const IMM3: u32>(a: m256i) -> u32 {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvpickve2gr_wu(transmute(a), IMM3)) }
}
```

## Block 519
**Metadata**: AST_ID=519 | TYPE=FUNCTION | NAME=lasx_xvpickve2gr_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickve2gr_d<const IMM2: u32>(a: m256i) -> i64 {
    static_assert_uimm_bits!(IMM2, 2);
    unsafe { transmute(__lasx_xvpickve2gr_d(transmute(a), IMM2)) }
}
```

## Block 520
**Metadata**: AST_ID=520 | TYPE=FUNCTION | NAME=lasx_xvpickve2gr_du | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickve2gr_du<const IMM2: u32>(a: m256i) -> u64 {
    static_assert_uimm_bits!(IMM2, 2);
    unsafe { transmute(__lasx_xvpickve2gr_du(transmute(a), IMM2)) }
}
```

## Block 521
**Metadata**: AST_ID=521 | TYPE=FUNCTION | NAME=lasx_xvaddwev_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_q_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_q_d(transmute(a), transmute(b))) }
}
```

## Block 522
**Metadata**: AST_ID=522 | TYPE=FUNCTION | NAME=lasx_xvaddwev_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_d_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_d_w(transmute(a), transmute(b))) }
}
```

## Block 523
**Metadata**: AST_ID=523 | TYPE=FUNCTION | NAME=lasx_xvaddwev_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_w_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_w_h(transmute(a), transmute(b))) }
}
```

## Block 524
**Metadata**: AST_ID=524 | TYPE=FUNCTION | NAME=lasx_xvaddwev_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_h_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_h_b(transmute(a), transmute(b))) }
}
```

## Block 525
**Metadata**: AST_ID=525 | TYPE=FUNCTION | NAME=lasx_xvaddwev_q_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_q_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_q_du(transmute(a), transmute(b))) }
}
```

## Block 526
**Metadata**: AST_ID=526 | TYPE=FUNCTION | NAME=lasx_xvaddwev_d_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_d_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_d_wu(transmute(a), transmute(b))) }
}
```

## Block 527
**Metadata**: AST_ID=527 | TYPE=FUNCTION | NAME=lasx_xvaddwev_w_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_w_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_w_hu(transmute(a), transmute(b))) }
}
```

## Block 528
**Metadata**: AST_ID=528 | TYPE=FUNCTION | NAME=lasx_xvaddwev_h_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_h_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_h_bu(transmute(a), transmute(b))) }
}
```

## Block 529
**Metadata**: AST_ID=529 | TYPE=FUNCTION | NAME=lasx_xvsubwev_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwev_q_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwev_q_d(transmute(a), transmute(b))) }
}
```

## Block 530
**Metadata**: AST_ID=530 | TYPE=FUNCTION | NAME=lasx_xvsubwev_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwev_d_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwev_d_w(transmute(a), transmute(b))) }
}
```

## Block 531
**Metadata**: AST_ID=531 | TYPE=FUNCTION | NAME=lasx_xvsubwev_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwev_w_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwev_w_h(transmute(a), transmute(b))) }
}
```

## Block 532
**Metadata**: AST_ID=532 | TYPE=FUNCTION | NAME=lasx_xvsubwev_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwev_h_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwev_h_b(transmute(a), transmute(b))) }
}
```

## Block 533
**Metadata**: AST_ID=533 | TYPE=FUNCTION | NAME=lasx_xvsubwev_q_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwev_q_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwev_q_du(transmute(a), transmute(b))) }
}
```

## Block 534
**Metadata**: AST_ID=534 | TYPE=FUNCTION | NAME=lasx_xvsubwev_d_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwev_d_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwev_d_wu(transmute(a), transmute(b))) }
}
```

## Block 535
**Metadata**: AST_ID=535 | TYPE=FUNCTION | NAME=lasx_xvsubwev_w_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwev_w_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwev_w_hu(transmute(a), transmute(b))) }
}
```

## Block 536
**Metadata**: AST_ID=536 | TYPE=FUNCTION | NAME=lasx_xvsubwev_h_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwev_h_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwev_h_bu(transmute(a), transmute(b))) }
}
```

## Block 537
**Metadata**: AST_ID=537 | TYPE=FUNCTION | NAME=lasx_xvmulwev_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_q_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_q_d(transmute(a), transmute(b))) }
}
```

## Block 538
**Metadata**: AST_ID=538 | TYPE=FUNCTION | NAME=lasx_xvmulwev_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_d_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_d_w(transmute(a), transmute(b))) }
}
```

## Block 539
**Metadata**: AST_ID=539 | TYPE=FUNCTION | NAME=lasx_xvmulwev_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_w_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_w_h(transmute(a), transmute(b))) }
}
```

## Block 540
**Metadata**: AST_ID=540 | TYPE=FUNCTION | NAME=lasx_xvmulwev_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_h_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_h_b(transmute(a), transmute(b))) }
}
```

## Block 541
**Metadata**: AST_ID=541 | TYPE=FUNCTION | NAME=lasx_xvmulwev_q_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_q_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_q_du(transmute(a), transmute(b))) }
}
```

## Block 542
**Metadata**: AST_ID=542 | TYPE=FUNCTION | NAME=lasx_xvmulwev_d_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_d_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_d_wu(transmute(a), transmute(b))) }
}
```

## Block 543
**Metadata**: AST_ID=543 | TYPE=FUNCTION | NAME=lasx_xvmulwev_w_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_w_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_w_hu(transmute(a), transmute(b))) }
}
```

## Block 544
**Metadata**: AST_ID=544 | TYPE=FUNCTION | NAME=lasx_xvmulwev_h_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_h_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_h_bu(transmute(a), transmute(b))) }
}
```

## Block 545
**Metadata**: AST_ID=545 | TYPE=FUNCTION | NAME=lasx_xvaddwod_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_q_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_q_d(transmute(a), transmute(b))) }
}
```

## Block 546
**Metadata**: AST_ID=546 | TYPE=FUNCTION | NAME=lasx_xvaddwod_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_d_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_d_w(transmute(a), transmute(b))) }
}
```

## Block 547
**Metadata**: AST_ID=547 | TYPE=FUNCTION | NAME=lasx_xvaddwod_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_w_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_w_h(transmute(a), transmute(b))) }
}
```

## Block 548
**Metadata**: AST_ID=548 | TYPE=FUNCTION | NAME=lasx_xvaddwod_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_h_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_h_b(transmute(a), transmute(b))) }
}
```

## Block 549
**Metadata**: AST_ID=549 | TYPE=FUNCTION | NAME=lasx_xvaddwod_q_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_q_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_q_du(transmute(a), transmute(b))) }
}
```

## Block 550
**Metadata**: AST_ID=550 | TYPE=FUNCTION | NAME=lasx_xvaddwod_d_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_d_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_d_wu(transmute(a), transmute(b))) }
}
```

## Block 551
**Metadata**: AST_ID=551 | TYPE=FUNCTION | NAME=lasx_xvaddwod_w_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_w_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_w_hu(transmute(a), transmute(b))) }
}
```

## Block 552
**Metadata**: AST_ID=552 | TYPE=FUNCTION | NAME=lasx_xvaddwod_h_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_h_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_h_bu(transmute(a), transmute(b))) }
}
```

## Block 553
**Metadata**: AST_ID=553 | TYPE=FUNCTION | NAME=lasx_xvsubwod_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwod_q_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwod_q_d(transmute(a), transmute(b))) }
}
```

## Block 554
**Metadata**: AST_ID=554 | TYPE=FUNCTION | NAME=lasx_xvsubwod_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwod_d_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwod_d_w(transmute(a), transmute(b))) }
}
```

## Block 555
**Metadata**: AST_ID=555 | TYPE=FUNCTION | NAME=lasx_xvsubwod_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwod_w_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwod_w_h(transmute(a), transmute(b))) }
}
```

## Block 556
**Metadata**: AST_ID=556 | TYPE=FUNCTION | NAME=lasx_xvsubwod_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwod_h_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwod_h_b(transmute(a), transmute(b))) }
}
```

## Block 557
**Metadata**: AST_ID=557 | TYPE=FUNCTION | NAME=lasx_xvsubwod_q_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwod_q_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwod_q_du(transmute(a), transmute(b))) }
}
```

## Block 558
**Metadata**: AST_ID=558 | TYPE=FUNCTION | NAME=lasx_xvsubwod_d_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwod_d_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwod_d_wu(transmute(a), transmute(b))) }
}
```

## Block 559
**Metadata**: AST_ID=559 | TYPE=FUNCTION | NAME=lasx_xvsubwod_w_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwod_w_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwod_w_hu(transmute(a), transmute(b))) }
}
```

## Block 560
**Metadata**: AST_ID=560 | TYPE=FUNCTION | NAME=lasx_xvsubwod_h_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsubwod_h_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsubwod_h_bu(transmute(a), transmute(b))) }
}
```

## Block 561
**Metadata**: AST_ID=561 | TYPE=FUNCTION | NAME=lasx_xvmulwod_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_q_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_q_d(transmute(a), transmute(b))) }
}
```

## Block 562
**Metadata**: AST_ID=562 | TYPE=FUNCTION | NAME=lasx_xvmulwod_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_d_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_d_w(transmute(a), transmute(b))) }
}
```

## Block 563
**Metadata**: AST_ID=563 | TYPE=FUNCTION | NAME=lasx_xvmulwod_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_w_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_w_h(transmute(a), transmute(b))) }
}
```

## Block 564
**Metadata**: AST_ID=564 | TYPE=FUNCTION | NAME=lasx_xvmulwod_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_h_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_h_b(transmute(a), transmute(b))) }
}
```

## Block 565
**Metadata**: AST_ID=565 | TYPE=FUNCTION | NAME=lasx_xvmulwod_q_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_q_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_q_du(transmute(a), transmute(b))) }
}
```

## Block 566
**Metadata**: AST_ID=566 | TYPE=FUNCTION | NAME=lasx_xvmulwod_d_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_d_wu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_d_wu(transmute(a), transmute(b))) }
}
```

## Block 567
**Metadata**: AST_ID=567 | TYPE=FUNCTION | NAME=lasx_xvmulwod_w_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_w_hu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_w_hu(transmute(a), transmute(b))) }
}
```

## Block 568
**Metadata**: AST_ID=568 | TYPE=FUNCTION | NAME=lasx_xvmulwod_h_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_h_bu(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_h_bu(transmute(a), transmute(b))) }
}
```

## Block 569
**Metadata**: AST_ID=569 | TYPE=FUNCTION | NAME=lasx_xvaddwev_d_wu_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_d_wu_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_d_wu_w(transmute(a), transmute(b))) }
}
```

## Block 570
**Metadata**: AST_ID=570 | TYPE=FUNCTION | NAME=lasx_xvaddwev_w_hu_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_w_hu_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_w_hu_h(transmute(a), transmute(b))) }
}
```

## Block 571
**Metadata**: AST_ID=571 | TYPE=FUNCTION | NAME=lasx_xvaddwev_h_bu_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_h_bu_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_h_bu_b(transmute(a), transmute(b))) }
}
```

## Block 572
**Metadata**: AST_ID=572 | TYPE=FUNCTION | NAME=lasx_xvmulwev_d_wu_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_d_wu_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_d_wu_w(transmute(a), transmute(b))) }
}
```

## Block 573
**Metadata**: AST_ID=573 | TYPE=FUNCTION | NAME=lasx_xvmulwev_w_hu_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_w_hu_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_w_hu_h(transmute(a), transmute(b))) }
}
```

## Block 574
**Metadata**: AST_ID=574 | TYPE=FUNCTION | NAME=lasx_xvmulwev_h_bu_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_h_bu_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_h_bu_b(transmute(a), transmute(b))) }
}
```

## Block 575
**Metadata**: AST_ID=575 | TYPE=FUNCTION | NAME=lasx_xvaddwod_d_wu_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_d_wu_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_d_wu_w(transmute(a), transmute(b))) }
}
```

## Block 576
**Metadata**: AST_ID=576 | TYPE=FUNCTION | NAME=lasx_xvaddwod_w_hu_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_w_hu_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_w_hu_h(transmute(a), transmute(b))) }
}
```

## Block 577
**Metadata**: AST_ID=577 | TYPE=FUNCTION | NAME=lasx_xvaddwod_h_bu_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_h_bu_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_h_bu_b(transmute(a), transmute(b))) }
}
```

## Block 578
**Metadata**: AST_ID=578 | TYPE=FUNCTION | NAME=lasx_xvmulwod_d_wu_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_d_wu_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_d_wu_w(transmute(a), transmute(b))) }
}
```

## Block 579
**Metadata**: AST_ID=579 | TYPE=FUNCTION | NAME=lasx_xvmulwod_w_hu_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_w_hu_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_w_hu_h(transmute(a), transmute(b))) }
}
```

## Block 580
**Metadata**: AST_ID=580 | TYPE=FUNCTION | NAME=lasx_xvmulwod_h_bu_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_h_bu_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_h_bu_b(transmute(a), transmute(b))) }
}
```

## Block 581
**Metadata**: AST_ID=581 | TYPE=FUNCTION | NAME=lasx_xvhaddw_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhaddw_q_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhaddw_q_d(transmute(a), transmute(b))) }
}
```

## Block 582
**Metadata**: AST_ID=582 | TYPE=FUNCTION | NAME=lasx_xvhaddw_qu_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhaddw_qu_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhaddw_qu_du(transmute(a), transmute(b))) }
}
```

## Block 583
**Metadata**: AST_ID=583 | TYPE=FUNCTION | NAME=lasx_xvhsubw_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhsubw_q_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhsubw_q_d(transmute(a), transmute(b))) }
}
```

## Block 584
**Metadata**: AST_ID=584 | TYPE=FUNCTION | NAME=lasx_xvhsubw_qu_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvhsubw_qu_du(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvhsubw_qu_du(transmute(a), transmute(b))) }
}
```

## Block 585
**Metadata**: AST_ID=585 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_q_d(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_q_d(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 586
**Metadata**: AST_ID=586 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_d_w(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_d_w(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 587
**Metadata**: AST_ID=587 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_w_h(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_w_h(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 588
**Metadata**: AST_ID=588 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_h_b(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_h_b(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 589
**Metadata**: AST_ID=589 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_q_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_q_du(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_q_du(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 590
**Metadata**: AST_ID=590 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_d_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_d_wu(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_d_wu(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 591
**Metadata**: AST_ID=591 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_w_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_w_hu(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_w_hu(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 592
**Metadata**: AST_ID=592 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_h_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_h_bu(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_h_bu(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 593
**Metadata**: AST_ID=593 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_q_d(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_q_d(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 594
**Metadata**: AST_ID=594 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_d_w(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_d_w(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 595
**Metadata**: AST_ID=595 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_w_h(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_w_h(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 596
**Metadata**: AST_ID=596 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_h_b(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_h_b(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 597
**Metadata**: AST_ID=597 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_q_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_q_du(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_q_du(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 598
**Metadata**: AST_ID=598 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_d_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_d_wu(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_d_wu(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 599
**Metadata**: AST_ID=599 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_w_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_w_hu(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_w_hu(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 600
**Metadata**: AST_ID=600 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_h_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_h_bu(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_h_bu(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 601
**Metadata**: AST_ID=601 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_q_du_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_q_du_d(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_q_du_d(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 602
**Metadata**: AST_ID=602 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_d_wu_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_d_wu_w(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_d_wu_w(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 603
**Metadata**: AST_ID=603 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_w_hu_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_w_hu_h(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_w_hu_h(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 604
**Metadata**: AST_ID=604 | TYPE=FUNCTION | NAME=lasx_xvmaddwev_h_bu_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwev_h_bu_b(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwev_h_bu_b(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 605
**Metadata**: AST_ID=605 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_q_du_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_q_du_d(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_q_du_d(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 606
**Metadata**: AST_ID=606 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_d_wu_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_d_wu_w(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_d_wu_w(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 607
**Metadata**: AST_ID=607 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_w_hu_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_w_hu_h(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_w_hu_h(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 608
**Metadata**: AST_ID=608 | TYPE=FUNCTION | NAME=lasx_xvmaddwod_h_bu_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmaddwod_h_bu_b(a: m256i, b: m256i, c: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmaddwod_h_bu_b(transmute(a), transmute(b), transmute(c))) }
}
```

## Block 609
**Metadata**: AST_ID=609 | TYPE=FUNCTION | NAME=lasx_xvrotr_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrotr_b(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvrotr_b(transmute(a), transmute(b))) }
}
```

## Block 610
**Metadata**: AST_ID=610 | TYPE=FUNCTION | NAME=lasx_xvrotr_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrotr_h(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvrotr_h(transmute(a), transmute(b))) }
}
```

## Block 611
**Metadata**: AST_ID=611 | TYPE=FUNCTION | NAME=lasx_xvrotr_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrotr_w(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvrotr_w(transmute(a), transmute(b))) }
}
```

## Block 612
**Metadata**: AST_ID=612 | TYPE=FUNCTION | NAME=lasx_xvrotr_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrotr_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvrotr_d(transmute(a), transmute(b))) }
}
```

## Block 613
**Metadata**: AST_ID=613 | TYPE=FUNCTION | NAME=lasx_xvadd_q | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvadd_q(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvadd_q(transmute(a), transmute(b))) }
}
```

## Block 614
**Metadata**: AST_ID=614 | TYPE=FUNCTION | NAME=lasx_xvsub_q | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsub_q(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvsub_q(transmute(a), transmute(b))) }
}
```

## Block 615
**Metadata**: AST_ID=615 | TYPE=FUNCTION | NAME=lasx_xvaddwev_q_du_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwev_q_du_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwev_q_du_d(transmute(a), transmute(b))) }
}
```

## Block 616
**Metadata**: AST_ID=616 | TYPE=FUNCTION | NAME=lasx_xvaddwod_q_du_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvaddwod_q_du_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvaddwod_q_du_d(transmute(a), transmute(b))) }
}
```

## Block 617
**Metadata**: AST_ID=617 | TYPE=FUNCTION | NAME=lasx_xvmulwev_q_du_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwev_q_du_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwev_q_du_d(transmute(a), transmute(b))) }
}
```

## Block 618
**Metadata**: AST_ID=618 | TYPE=FUNCTION | NAME=lasx_xvmulwod_q_du_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmulwod_q_du_d(a: m256i, b: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmulwod_q_du_d(transmute(a), transmute(b))) }
}
```

## Block 619
**Metadata**: AST_ID=619 | TYPE=FUNCTION | NAME=lasx_xvmskgez_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmskgez_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmskgez_b(transmute(a))) }
}
```

## Block 620
**Metadata**: AST_ID=620 | TYPE=FUNCTION | NAME=lasx_xvmsknz_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvmsknz_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvmsknz_b(transmute(a))) }
}
```

## Block 621
**Metadata**: AST_ID=621 | TYPE=FUNCTION | NAME=lasx_xvexth_h_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvexth_h_b(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvexth_h_b(transmute(a))) }
}
```

## Block 622
**Metadata**: AST_ID=622 | TYPE=FUNCTION | NAME=lasx_xvexth_w_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvexth_w_h(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvexth_w_h(transmute(a))) }
}
```

## Block 623
**Metadata**: AST_ID=623 | TYPE=FUNCTION | NAME=lasx_xvexth_d_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvexth_d_w(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvexth_d_w(transmute(a))) }
}
```

## Block 624
**Metadata**: AST_ID=624 | TYPE=FUNCTION | NAME=lasx_xvexth_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvexth_q_d(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvexth_q_d(transmute(a))) }
}
```

## Block 625
**Metadata**: AST_ID=625 | TYPE=FUNCTION | NAME=lasx_xvexth_hu_bu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvexth_hu_bu(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvexth_hu_bu(transmute(a))) }
}
```

## Block 626
**Metadata**: AST_ID=626 | TYPE=FUNCTION | NAME=lasx_xvexth_wu_hu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvexth_wu_hu(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvexth_wu_hu(transmute(a))) }
}
```

## Block 627
**Metadata**: AST_ID=627 | TYPE=FUNCTION | NAME=lasx_xvexth_du_wu | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvexth_du_wu(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvexth_du_wu(transmute(a))) }
}
```

## Block 628
**Metadata**: AST_ID=628 | TYPE=FUNCTION | NAME=lasx_xvexth_qu_du | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvexth_qu_du(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvexth_qu_du(transmute(a))) }
}
```

## Block 629
**Metadata**: AST_ID=629 | TYPE=FUNCTION | NAME=lasx_xvrotri_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrotri_b<const IMM3: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvrotri_b(transmute(a), IMM3)) }
}
```

## Block 630
**Metadata**: AST_ID=630 | TYPE=FUNCTION | NAME=lasx_xvrotri_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrotri_h<const IMM4: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvrotri_h(transmute(a), IMM4)) }
}
```

## Block 631
**Metadata**: AST_ID=631 | TYPE=FUNCTION | NAME=lasx_xvrotri_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrotri_w<const IMM5: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvrotri_w(transmute(a), IMM5)) }
}
```

## Block 632
**Metadata**: AST_ID=632 | TYPE=FUNCTION | NAME=lasx_xvrotri_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrotri_d<const IMM6: u32>(a: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvrotri_d(transmute(a), IMM6)) }
}
```

## Block 633
**Metadata**: AST_ID=633 | TYPE=FUNCTION | NAME=lasx_xvextl_q_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvextl_q_d(a: m256i) -> m256i {
    unsafe { transmute(__lasx_xvextl_q_d(transmute(a))) }
}
```

## Block 634
**Metadata**: AST_ID=634 | TYPE=FUNCTION | NAME=lasx_xvsrlni_b_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlni_b_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsrlni_b_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 635
**Metadata**: AST_ID=635 | TYPE=FUNCTION | NAME=lasx_xvsrlni_h_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlni_h_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsrlni_h_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 636
**Metadata**: AST_ID=636 | TYPE=FUNCTION | NAME=lasx_xvsrlni_w_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlni_w_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvsrlni_w_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 637
**Metadata**: AST_ID=637 | TYPE=FUNCTION | NAME=lasx_xvsrlni_d_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlni_d_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvsrlni_d_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 638
**Metadata**: AST_ID=638 | TYPE=FUNCTION | NAME=lasx_xvsrlrni_b_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlrni_b_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsrlrni_b_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 639
**Metadata**: AST_ID=639 | TYPE=FUNCTION | NAME=lasx_xvsrlrni_h_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlrni_h_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsrlrni_h_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 640
**Metadata**: AST_ID=640 | TYPE=FUNCTION | NAME=lasx_xvsrlrni_w_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlrni_w_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvsrlrni_w_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 641
**Metadata**: AST_ID=641 | TYPE=FUNCTION | NAME=lasx_xvsrlrni_d_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrlrni_d_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvsrlrni_d_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 642
**Metadata**: AST_ID=642 | TYPE=FUNCTION | NAME=lasx_xvssrlni_b_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlni_b_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvssrlni_b_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 643
**Metadata**: AST_ID=643 | TYPE=FUNCTION | NAME=lasx_xvssrlni_h_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlni_h_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvssrlni_h_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 644
**Metadata**: AST_ID=644 | TYPE=FUNCTION | NAME=lasx_xvssrlni_w_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlni_w_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvssrlni_w_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 645
**Metadata**: AST_ID=645 | TYPE=FUNCTION | NAME=lasx_xvssrlni_d_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlni_d_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvssrlni_d_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 646
**Metadata**: AST_ID=646 | TYPE=FUNCTION | NAME=lasx_xvssrlni_bu_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlni_bu_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvssrlni_bu_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 647
**Metadata**: AST_ID=647 | TYPE=FUNCTION | NAME=lasx_xvssrlni_hu_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlni_hu_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvssrlni_hu_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 648
**Metadata**: AST_ID=648 | TYPE=FUNCTION | NAME=lasx_xvssrlni_wu_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlni_wu_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvssrlni_wu_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 649
**Metadata**: AST_ID=649 | TYPE=FUNCTION | NAME=lasx_xvssrlni_du_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlni_du_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvssrlni_du_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 650
**Metadata**: AST_ID=650 | TYPE=FUNCTION | NAME=lasx_xvssrlrni_b_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrni_b_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvssrlrni_b_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 651
**Metadata**: AST_ID=651 | TYPE=FUNCTION | NAME=lasx_xvssrlrni_h_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrni_h_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvssrlrni_h_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 652
**Metadata**: AST_ID=652 | TYPE=FUNCTION | NAME=lasx_xvssrlrni_w_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrni_w_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvssrlrni_w_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 653
**Metadata**: AST_ID=653 | TYPE=FUNCTION | NAME=lasx_xvssrlrni_d_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrni_d_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvssrlrni_d_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 654
**Metadata**: AST_ID=654 | TYPE=FUNCTION | NAME=lasx_xvssrlrni_bu_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrni_bu_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvssrlrni_bu_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 655
**Metadata**: AST_ID=655 | TYPE=FUNCTION | NAME=lasx_xvssrlrni_hu_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrni_hu_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvssrlrni_hu_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 656
**Metadata**: AST_ID=656 | TYPE=FUNCTION | NAME=lasx_xvssrlrni_wu_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrni_wu_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvssrlrni_wu_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 657
**Metadata**: AST_ID=657 | TYPE=FUNCTION | NAME=lasx_xvssrlrni_du_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrlrni_du_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvssrlrni_du_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 658
**Metadata**: AST_ID=658 | TYPE=FUNCTION | NAME=lasx_xvsrani_b_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrani_b_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsrani_b_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 659
**Metadata**: AST_ID=659 | TYPE=FUNCTION | NAME=lasx_xvsrani_h_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrani_h_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsrani_h_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 660
**Metadata**: AST_ID=660 | TYPE=FUNCTION | NAME=lasx_xvsrani_w_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrani_w_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvsrani_w_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 661
**Metadata**: AST_ID=661 | TYPE=FUNCTION | NAME=lasx_xvsrani_d_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrani_d_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvsrani_d_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 662
**Metadata**: AST_ID=662 | TYPE=FUNCTION | NAME=lasx_xvsrarni_b_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrarni_b_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvsrarni_b_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 663
**Metadata**: AST_ID=663 | TYPE=FUNCTION | NAME=lasx_xvsrarni_h_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrarni_h_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvsrarni_h_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 664
**Metadata**: AST_ID=664 | TYPE=FUNCTION | NAME=lasx_xvsrarni_w_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrarni_w_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvsrarni_w_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 665
**Metadata**: AST_ID=665 | TYPE=FUNCTION | NAME=lasx_xvsrarni_d_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvsrarni_d_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvsrarni_d_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 666
**Metadata**: AST_ID=666 | TYPE=FUNCTION | NAME=lasx_xvssrani_b_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrani_b_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvssrani_b_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 667
**Metadata**: AST_ID=667 | TYPE=FUNCTION | NAME=lasx_xvssrani_h_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrani_h_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvssrani_h_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 668
**Metadata**: AST_ID=668 | TYPE=FUNCTION | NAME=lasx_xvssrani_w_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrani_w_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvssrani_w_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 669
**Metadata**: AST_ID=669 | TYPE=FUNCTION | NAME=lasx_xvssrani_d_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrani_d_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvssrani_d_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 670
**Metadata**: AST_ID=670 | TYPE=FUNCTION | NAME=lasx_xvssrani_bu_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrani_bu_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvssrani_bu_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 671
**Metadata**: AST_ID=671 | TYPE=FUNCTION | NAME=lasx_xvssrani_hu_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrani_hu_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvssrani_hu_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 672
**Metadata**: AST_ID=672 | TYPE=FUNCTION | NAME=lasx_xvssrani_wu_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrani_wu_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvssrani_wu_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 673
**Metadata**: AST_ID=673 | TYPE=FUNCTION | NAME=lasx_xvssrani_du_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrani_du_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvssrani_du_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 674
**Metadata**: AST_ID=674 | TYPE=FUNCTION | NAME=lasx_xvssrarni_b_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarni_b_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvssrarni_b_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 675
**Metadata**: AST_ID=675 | TYPE=FUNCTION | NAME=lasx_xvssrarni_h_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarni_h_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvssrarni_h_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 676
**Metadata**: AST_ID=676 | TYPE=FUNCTION | NAME=lasx_xvssrarni_w_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarni_w_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvssrarni_w_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 677
**Metadata**: AST_ID=677 | TYPE=FUNCTION | NAME=lasx_xvssrarni_d_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarni_d_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvssrarni_d_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 678
**Metadata**: AST_ID=678 | TYPE=FUNCTION | NAME=lasx_xvssrarni_bu_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarni_bu_h<const IMM4: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM4, 4);
    unsafe { transmute(__lasx_xvssrarni_bu_h(transmute(a), transmute(b), IMM4)) }
}
```

## Block 679
**Metadata**: AST_ID=679 | TYPE=FUNCTION | NAME=lasx_xvssrarni_hu_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarni_hu_w<const IMM5: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM5, 5);
    unsafe { transmute(__lasx_xvssrarni_hu_w(transmute(a), transmute(b), IMM5)) }
}
```

## Block 680
**Metadata**: AST_ID=680 | TYPE=FUNCTION | NAME=lasx_xvssrarni_wu_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarni_wu_d<const IMM6: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM6, 6);
    unsafe { transmute(__lasx_xvssrarni_wu_d(transmute(a), transmute(b), IMM6)) }
}
```

## Block 681
**Metadata**: AST_ID=681 | TYPE=FUNCTION | NAME=lasx_xvssrarni_du_q | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvssrarni_du_q<const IMM7: u32>(a: m256i, b: m256i) -> m256i {
    static_assert_uimm_bits!(IMM7, 7);
    unsafe { transmute(__lasx_xvssrarni_du_q(transmute(a), transmute(b), IMM7)) }
}
```

## Block 682
**Metadata**: AST_ID=682 | TYPE=FUNCTION | NAME=lasx_xbnz_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xbnz_b(a: m256i) -> i32 {
    unsafe { transmute(__lasx_xbnz_b(transmute(a))) }
}
```

## Block 683
**Metadata**: AST_ID=683 | TYPE=FUNCTION | NAME=lasx_xbnz_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xbnz_d(a: m256i) -> i32 {
    unsafe { transmute(__lasx_xbnz_d(transmute(a))) }
}
```

## Block 684
**Metadata**: AST_ID=684 | TYPE=FUNCTION | NAME=lasx_xbnz_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xbnz_h(a: m256i) -> i32 {
    unsafe { transmute(__lasx_xbnz_h(transmute(a))) }
}
```

## Block 685
**Metadata**: AST_ID=685 | TYPE=FUNCTION | NAME=lasx_xbnz_v | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xbnz_v(a: m256i) -> i32 {
    unsafe { transmute(__lasx_xbnz_v(transmute(a))) }
}
```

## Block 686
**Metadata**: AST_ID=686 | TYPE=FUNCTION | NAME=lasx_xbnz_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xbnz_w(a: m256i) -> i32 {
    unsafe { transmute(__lasx_xbnz_w(transmute(a))) }
}
```

## Block 687
**Metadata**: AST_ID=687 | TYPE=FUNCTION | NAME=lasx_xbz_b | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xbz_b(a: m256i) -> i32 {
    unsafe { transmute(__lasx_xbz_b(transmute(a))) }
}
```

## Block 688
**Metadata**: AST_ID=688 | TYPE=FUNCTION | NAME=lasx_xbz_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xbz_d(a: m256i) -> i32 {
    unsafe { transmute(__lasx_xbz_d(transmute(a))) }
}
```

## Block 689
**Metadata**: AST_ID=689 | TYPE=FUNCTION | NAME=lasx_xbz_h | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xbz_h(a: m256i) -> i32 {
    unsafe { transmute(__lasx_xbz_h(transmute(a))) }
}
```

## Block 690
**Metadata**: AST_ID=690 | TYPE=FUNCTION | NAME=lasx_xbz_v | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xbz_v(a: m256i) -> i32 {
    unsafe { transmute(__lasx_xbz_v(transmute(a))) }
}
```

## Block 691
**Metadata**: AST_ID=691 | TYPE=FUNCTION | NAME=lasx_xbz_w | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xbz_w(a: m256i) -> i32 {
    unsafe { transmute(__lasx_xbz_w(transmute(a))) }
}
```

## Block 692
**Metadata**: AST_ID=692 | TYPE=FUNCTION | NAME=lasx_xvfcmp_caf_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_caf_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_caf_d(transmute(a), transmute(b))) }
}
```

## Block 693
**Metadata**: AST_ID=693 | TYPE=FUNCTION | NAME=lasx_xvfcmp_caf_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_caf_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_caf_s(transmute(a), transmute(b))) }
}
```

## Block 694
**Metadata**: AST_ID=694 | TYPE=FUNCTION | NAME=lasx_xvfcmp_ceq_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_ceq_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_ceq_d(transmute(a), transmute(b))) }
}
```

## Block 695
**Metadata**: AST_ID=695 | TYPE=FUNCTION | NAME=lasx_xvfcmp_ceq_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_ceq_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_ceq_s(transmute(a), transmute(b))) }
}
```

## Block 696
**Metadata**: AST_ID=696 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cle_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cle_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cle_d(transmute(a), transmute(b))) }
}
```

## Block 697
**Metadata**: AST_ID=697 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cle_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cle_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cle_s(transmute(a), transmute(b))) }
}
```

## Block 698
**Metadata**: AST_ID=698 | TYPE=FUNCTION | NAME=lasx_xvfcmp_clt_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_clt_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_clt_d(transmute(a), transmute(b))) }
}
```

## Block 699
**Metadata**: AST_ID=699 | TYPE=FUNCTION | NAME=lasx_xvfcmp_clt_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_clt_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_clt_s(transmute(a), transmute(b))) }
}
```

## Block 700
**Metadata**: AST_ID=700 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cne_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cne_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cne_d(transmute(a), transmute(b))) }
}
```

## Block 701
**Metadata**: AST_ID=701 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cne_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cne_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cne_s(transmute(a), transmute(b))) }
}
```

## Block 702
**Metadata**: AST_ID=702 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cor_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cor_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cor_d(transmute(a), transmute(b))) }
}
```

## Block 703
**Metadata**: AST_ID=703 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cor_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cor_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cor_s(transmute(a), transmute(b))) }
}
```

## Block 704
**Metadata**: AST_ID=704 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cueq_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cueq_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cueq_d(transmute(a), transmute(b))) }
}
```

## Block 705
**Metadata**: AST_ID=705 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cueq_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cueq_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cueq_s(transmute(a), transmute(b))) }
}
```

## Block 706
**Metadata**: AST_ID=706 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cule_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cule_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cule_d(transmute(a), transmute(b))) }
}
```

## Block 707
**Metadata**: AST_ID=707 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cule_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cule_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cule_s(transmute(a), transmute(b))) }
}
```

## Block 708
**Metadata**: AST_ID=708 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cult_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cult_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cult_d(transmute(a), transmute(b))) }
}
```

## Block 709
**Metadata**: AST_ID=709 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cult_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cult_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cult_s(transmute(a), transmute(b))) }
}
```

## Block 710
**Metadata**: AST_ID=710 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cun_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cun_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cun_d(transmute(a), transmute(b))) }
}
```

## Block 711
**Metadata**: AST_ID=711 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cune_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cune_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cune_d(transmute(a), transmute(b))) }
}
```

## Block 712
**Metadata**: AST_ID=712 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cune_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cune_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cune_s(transmute(a), transmute(b))) }
}
```

## Block 713
**Metadata**: AST_ID=713 | TYPE=FUNCTION | NAME=lasx_xvfcmp_cun_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_cun_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_cun_s(transmute(a), transmute(b))) }
}
```

## Block 714
**Metadata**: AST_ID=714 | TYPE=FUNCTION | NAME=lasx_xvfcmp_saf_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_saf_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_saf_d(transmute(a), transmute(b))) }
}
```

## Block 715
**Metadata**: AST_ID=715 | TYPE=FUNCTION | NAME=lasx_xvfcmp_saf_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_saf_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_saf_s(transmute(a), transmute(b))) }
}
```

## Block 716
**Metadata**: AST_ID=716 | TYPE=FUNCTION | NAME=lasx_xvfcmp_seq_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_seq_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_seq_d(transmute(a), transmute(b))) }
}
```

## Block 717
**Metadata**: AST_ID=717 | TYPE=FUNCTION | NAME=lasx_xvfcmp_seq_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_seq_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_seq_s(transmute(a), transmute(b))) }
}
```

## Block 718
**Metadata**: AST_ID=718 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sle_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sle_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sle_d(transmute(a), transmute(b))) }
}
```

## Block 719
**Metadata**: AST_ID=719 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sle_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sle_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sle_s(transmute(a), transmute(b))) }
}
```

## Block 720
**Metadata**: AST_ID=720 | TYPE=FUNCTION | NAME=lasx_xvfcmp_slt_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_slt_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_slt_d(transmute(a), transmute(b))) }
}
```

## Block 721
**Metadata**: AST_ID=721 | TYPE=FUNCTION | NAME=lasx_xvfcmp_slt_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_slt_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_slt_s(transmute(a), transmute(b))) }
}
```

## Block 722
**Metadata**: AST_ID=722 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sne_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sne_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sne_d(transmute(a), transmute(b))) }
}
```

## Block 723
**Metadata**: AST_ID=723 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sne_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sne_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sne_s(transmute(a), transmute(b))) }
}
```

## Block 724
**Metadata**: AST_ID=724 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sor_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sor_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sor_d(transmute(a), transmute(b))) }
}
```

## Block 725
**Metadata**: AST_ID=725 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sor_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sor_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sor_s(transmute(a), transmute(b))) }
}
```

## Block 726
**Metadata**: AST_ID=726 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sueq_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sueq_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sueq_d(transmute(a), transmute(b))) }
}
```

## Block 727
**Metadata**: AST_ID=727 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sueq_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sueq_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sueq_s(transmute(a), transmute(b))) }
}
```

## Block 728
**Metadata**: AST_ID=728 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sule_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sule_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sule_d(transmute(a), transmute(b))) }
}
```

## Block 729
**Metadata**: AST_ID=729 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sule_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sule_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sule_s(transmute(a), transmute(b))) }
}
```

## Block 730
**Metadata**: AST_ID=730 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sult_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sult_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sult_d(transmute(a), transmute(b))) }
}
```

## Block 731
**Metadata**: AST_ID=731 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sult_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sult_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sult_s(transmute(a), transmute(b))) }
}
```

## Block 732
**Metadata**: AST_ID=732 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sun_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sun_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sun_d(transmute(a), transmute(b))) }
}
```

## Block 733
**Metadata**: AST_ID=733 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sune_d | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sune_d(a: m256d, b: m256d) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sune_d(transmute(a), transmute(b))) }
}
```

## Block 734
**Metadata**: AST_ID=734 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sune_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sune_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sune_s(transmute(a), transmute(b))) }
}
```

## Block 735
**Metadata**: AST_ID=735 | TYPE=FUNCTION | NAME=lasx_xvfcmp_sun_s | COMPLEXITY=7 | LINES=7

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvfcmp_sun_s(a: m256, b: m256) -> m256i {
    unsafe { transmute(__lasx_xvfcmp_sun_s(transmute(a), transmute(b))) }
}
```

## Block 736
**Metadata**: AST_ID=736 | TYPE=FUNCTION | NAME=lasx_xvpickve_d_f | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickve_d_f<const IMM2: u32>(a: m256d) -> m256d {
    static_assert_uimm_bits!(IMM2, 2);
    unsafe { transmute(__lasx_xvpickve_d_f(transmute(a), IMM2)) }
}
```

## Block 737
**Metadata**: AST_ID=737 | TYPE=FUNCTION | NAME=lasx_xvpickve_w_f | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvpickve_w_f<const IMM3: u32>(a: m256) -> m256 {
    static_assert_uimm_bits!(IMM3, 3);
    unsafe { transmute(__lasx_xvpickve_w_f(transmute(a), IMM3)) }
}
```

## Block 738
**Metadata**: AST_ID=738 | TYPE=FUNCTION | NAME=lasx_xvrepli_b | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(0)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrepli_b<const IMM_S10: i32>() -> m256i {
    static_assert_simm_bits!(IMM_S10, 10);
    unsafe { transmute(__lasx_xvrepli_b(IMM_S10)) }
}
```

## Block 739
**Metadata**: AST_ID=739 | TYPE=FUNCTION | NAME=lasx_xvrepli_d | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(0)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrepli_d<const IMM_S10: i32>() -> m256i {
    static_assert_simm_bits!(IMM_S10, 10);
    unsafe { transmute(__lasx_xvrepli_d(IMM_S10)) }
}
```

## Block 740
**Metadata**: AST_ID=740 | TYPE=FUNCTION | NAME=lasx_xvrepli_h | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(0)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrepli_h<const IMM_S10: i32>() -> m256i {
    static_assert_simm_bits!(IMM_S10, 10);
    unsafe { transmute(__lasx_xvrepli_h(IMM_S10)) }
}
```

## Block 741
**Metadata**: AST_ID=741 | TYPE=FUNCTION | NAME=lasx_xvrepli_w | COMPLEXITY=7 | LINES=9

```rust
#[inline]
#[target_feature(enable = "lasx")]
#[rustc_legacy_const_generics(0)]
#[unstable(feature = "stdarch_loongarch", issue = "117427")]
pub fn lasx_xvrepli_w<const IMM_S10: i32>() -> m256i {
    static_assert_simm_bits!(IMM_S10, 10);
    unsafe { transmute(__lasx_xvrepli_w(IMM_S10)) }
}
```

---
*Generated by AST tracing system*
