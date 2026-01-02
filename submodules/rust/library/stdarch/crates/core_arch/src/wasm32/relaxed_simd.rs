mkuse!{use super :: v128 ;}
mkuse!{use crate :: core_arch :: simd ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "unadjusted" { # [link_name = "llvm.wasm.relaxed.swizzle"] fn llvm_relaxed_swizzle (a : simd :: i8x16 , b : simd :: i8x16) -> simd :: i8x16 ; # [link_name = "llvm.wasm.relaxed.trunc.signed"] fn llvm_relaxed_trunc_signed (a : simd :: f32x4) -> simd :: i32x4 ; # [link_name = "llvm.wasm.relaxed.trunc.unsigned"] fn llvm_relaxed_trunc_unsigned (a : simd :: f32x4) -> simd :: i32x4 ; # [link_name = "llvm.wasm.relaxed.trunc.signed.zero"] fn llvm_relaxed_trunc_signed_zero (a : simd :: f64x2) -> simd :: i32x4 ; # [link_name = "llvm.wasm.relaxed.trunc.unsigned.zero"] fn llvm_relaxed_trunc_unsigned_zero (a : simd :: f64x2) -> simd :: i32x4 ; # [link_name = "llvm.wasm.relaxed.madd.v4f32"] fn llvm_f32x4_fma (a : simd :: f32x4 , b : simd :: f32x4 , c : simd :: f32x4) -> simd :: f32x4 ; # [link_name = "llvm.wasm.relaxed.nmadd.v4f32"] fn llvm_f32x4_fms (a : simd :: f32x4 , b : simd :: f32x4 , c : simd :: f32x4) -> simd :: f32x4 ; # [link_name = "llvm.wasm.relaxed.madd.v2f64"] fn llvm_f64x2_fma (a : simd :: f64x2 , b : simd :: f64x2 , c : simd :: f64x2) -> simd :: f64x2 ; # [link_name = "llvm.wasm.relaxed.nmadd.v2f64"] fn llvm_f64x2_fms (a : simd :: f64x2 , b : simd :: f64x2 , c : simd :: f64x2) -> simd :: f64x2 ; # [link_name = "llvm.wasm.relaxed.laneselect.v16i8"] fn llvm_i8x16_laneselect (a : simd :: i8x16 , b : simd :: i8x16 , c : simd :: i8x16) -> simd :: i8x16 ; # [link_name = "llvm.wasm.relaxed.laneselect.v8i16"] fn llvm_i16x8_laneselect (a : simd :: i16x8 , b : simd :: i16x8 , c : simd :: i16x8) -> simd :: i16x8 ; # [link_name = "llvm.wasm.relaxed.laneselect.v4i32"] fn llvm_i32x4_laneselect (a : simd :: i32x4 , b : simd :: i32x4 , c : simd :: i32x4) -> simd :: i32x4 ; # [link_name = "llvm.wasm.relaxed.laneselect.v2i64"] fn llvm_i64x2_laneselect (a : simd :: i64x2 , b : simd :: i64x2 , c : simd :: i64x2) -> simd :: i64x2 ; # [link_name = "llvm.wasm.relaxed.min.v4f32"] fn llvm_f32x4_relaxed_min (a : simd :: f32x4 , b : simd :: f32x4) -> simd :: f32x4 ; # [link_name = "llvm.wasm.relaxed.min.v2f64"] fn llvm_f64x2_relaxed_min (a : simd :: f64x2 , b : simd :: f64x2) -> simd :: f64x2 ; # [link_name = "llvm.wasm.relaxed.max.v4f32"] fn llvm_f32x4_relaxed_max (a : simd :: f32x4 , b : simd :: f32x4) -> simd :: f32x4 ; # [link_name = "llvm.wasm.relaxed.max.v2f64"] fn llvm_f64x2_relaxed_max (a : simd :: f64x2 , b : simd :: f64x2) -> simd :: f64x2 ; # [link_name = "llvm.wasm.relaxed.q15mulr.signed"] fn llvm_relaxed_q15mulr_signed (a : simd :: i16x8 , b : simd :: i16x8) -> simd :: i16x8 ; # [link_name = "llvm.wasm.relaxed.dot.i8x16.i7x16.signed"] fn llvm_i16x8_relaxed_dot_i8x16_i7x16_s (a : simd :: i8x16 , b : simd :: i8x16) -> simd :: i16x8 ; # [link_name = "llvm.wasm.relaxed.dot.i8x16.i7x16.add.signed"] fn llvm_i32x4_relaxed_dot_i8x16_i7x16_add_s (a : simd :: i8x16 , b : simd :: i8x16 , c : simd :: i32x4 ,) -> simd :: i32x4 ; }}

macro_rules! i8x16_relaxed_swizzle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i8x16_relaxed_swizzle in module {}", module_path!());
    };
}

mkfn!{
    i8x16_relaxed_swizzle_introspect!();
    # [doc = " A relaxed version of `i8x16_swizzle(a, s)` which selects lanes from `a`"] # [doc = " using indices in `s`."] # [doc = ""] # [doc = " Indices in the range `[0,15]` will select the `i`-th element of `a`."] # [doc = " If the high bit of any element of `s` is set (meaning 128 or greater) then"] # [doc = " the corresponding output lane is guaranteed to be zero. Otherwise if the"] # [doc = " element of `s` is within the range `[16,128)` then the output lane is either"] # [doc = " 0 or `a[s[i] % 16]` depending on the implementation."] # [inline] # [cfg_attr (test , assert_instr (i8x16 . relaxed_swizzle))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i8x16.relaxed_swizzle"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn i8x16_relaxed_swizzle (a : v128 , s : v128) -> v128 { unsafe { llvm_relaxed_swizzle (a . as_i8x16 () , s . as_i8x16 ()) . v128 () } }
}
mkuse!{# [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub use i8x16_relaxed_swizzle as u8x16_relaxed_swizzle ;}

macro_rules! i32x4_relaxed_trunc_f32x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i32x4_relaxed_trunc_f32x4 in module {}", module_path!());
    };
}

mkfn!{
    i32x4_relaxed_trunc_f32x4_introspect!();
    # [doc = " A relaxed version of `i32x4_trunc_sat_f32x4(a)` converts the `f32` lanes"] # [doc = " of `a` to signed 32-bit integers."] # [doc = ""] # [doc = " Values which don't fit in 32-bit integers or are NaN may have the same"] # [doc = " result as `i32x4_trunc_sat_f32x4` or may return `i32::MIN`."] # [inline] # [cfg_attr (test , assert_instr (i32x4 . relaxed_trunc_f32x4_s))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i32x4.relaxed_trunc_f32x4_s"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn i32x4_relaxed_trunc_f32x4 (a : v128) -> v128 { unsafe { llvm_relaxed_trunc_signed (a . as_f32x4 ()) . v128 () } }
}

macro_rules! u32x4_relaxed_trunc_f32x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u32x4_relaxed_trunc_f32x4 in module {}", module_path!());
    };
}

mkfn!{
    u32x4_relaxed_trunc_f32x4_introspect!();
    # [doc = " A relaxed version of `u32x4_trunc_sat_f32x4(a)` converts the `f32` lanes"] # [doc = " of `a` to unsigned 32-bit integers."] # [doc = ""] # [doc = " Values which don't fit in 32-bit unsigned integers or are NaN may have the"] # [doc = " same result as `u32x4_trunc_sat_f32x4` or may return `u32::MAX`."] # [inline] # [cfg_attr (test , assert_instr (i32x4 . relaxed_trunc_f32x4_u))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i32x4.relaxed_trunc_f32x4_u"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn u32x4_relaxed_trunc_f32x4 (a : v128) -> v128 { unsafe { llvm_relaxed_trunc_unsigned (a . as_f32x4 ()) . v128 () } }
}

macro_rules! i32x4_relaxed_trunc_f64x2_zero_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i32x4_relaxed_trunc_f64x2_zero in module {}", module_path!());
    };
}

mkfn!{
    i32x4_relaxed_trunc_f64x2_zero_introspect!();
    # [doc = " A relaxed version of `i32x4_trunc_sat_f64x2_zero(a)` converts the `f64`"] # [doc = " lanes of `a` to signed 32-bit integers and the upper two lanes are zero."] # [doc = ""] # [doc = " Values which don't fit in 32-bit integers or are NaN may have the same"] # [doc = " result as `i32x4_trunc_sat_f32x4` or may return `i32::MIN`."] # [inline] # [cfg_attr (test , assert_instr (i32x4 . relaxed_trunc_f64x2_s_zero))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i32x4.relaxed_trunc_f64x2_s_zero"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn i32x4_relaxed_trunc_f64x2_zero (a : v128) -> v128 { unsafe { llvm_relaxed_trunc_signed_zero (a . as_f64x2 ()) . v128 () } }
}

macro_rules! u32x4_relaxed_trunc_f64x2_zero_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u32x4_relaxed_trunc_f64x2_zero in module {}", module_path!());
    };
}

mkfn!{
    u32x4_relaxed_trunc_f64x2_zero_introspect!();
    # [doc = " A relaxed version of `u32x4_trunc_sat_f64x2_zero(a)` converts the `f64`"] # [doc = " lanes of `a` to unsigned 32-bit integers and the upper two lanes are zero."] # [doc = ""] # [doc = " Values which don't fit in 32-bit unsigned integers or are NaN may have the"] # [doc = " same result as `u32x4_trunc_sat_f32x4` or may return `u32::MAX`."] # [inline] # [cfg_attr (test , assert_instr (i32x4 . relaxed_trunc_f64x2_u_zero))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i32x4.relaxed_trunc_f64x2_u_zero"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn u32x4_relaxed_trunc_f64x2_zero (a : v128) -> v128 { unsafe { llvm_relaxed_trunc_unsigned_zero (a . as_f64x2 ()) . v128 () } }
}

macro_rules! f32x4_relaxed_madd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f32x4_relaxed_madd in module {}", module_path!());
    };
}

mkfn!{
    f32x4_relaxed_madd_introspect!();
    # [doc = " Computes `a * b + c` with either one rounding or two roundings."] # [inline] # [cfg_attr (test , assert_instr (f32x4 . relaxed_madd))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("f32x4.relaxed_madd"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn f32x4_relaxed_madd (a : v128 , b : v128 , c : v128) -> v128 { unsafe { llvm_f32x4_fma (a . as_f32x4 () , b . as_f32x4 () , c . as_f32x4 ()) . v128 () } }
}

macro_rules! f32x4_relaxed_nmadd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f32x4_relaxed_nmadd in module {}", module_path!());
    };
}

mkfn!{
    f32x4_relaxed_nmadd_introspect!();
    # [doc = " Computes `-a * b + c` with either one rounding or two roundings."] # [inline] # [cfg_attr (test , assert_instr (f32x4 . relaxed_nmadd))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("f32x4.relaxed_nmadd"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn f32x4_relaxed_nmadd (a : v128 , b : v128 , c : v128) -> v128 { unsafe { llvm_f32x4_fms (a . as_f32x4 () , b . as_f32x4 () , c . as_f32x4 ()) . v128 () } }
}

macro_rules! f64x2_relaxed_madd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f64x2_relaxed_madd in module {}", module_path!());
    };
}

mkfn!{
    f64x2_relaxed_madd_introspect!();
    # [doc = " Computes `a * b + c` with either one rounding or two roundings."] # [inline] # [cfg_attr (test , assert_instr (f64x2 . relaxed_madd))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("f64x2.relaxed_madd"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn f64x2_relaxed_madd (a : v128 , b : v128 , c : v128) -> v128 { unsafe { llvm_f64x2_fma (a . as_f64x2 () , b . as_f64x2 () , c . as_f64x2 ()) . v128 () } }
}

macro_rules! f64x2_relaxed_nmadd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f64x2_relaxed_nmadd in module {}", module_path!());
    };
}

mkfn!{
    f64x2_relaxed_nmadd_introspect!();
    # [doc = " Computes `-a * b + c` with either one rounding or two roundings."] # [inline] # [cfg_attr (test , assert_instr (f64x2 . relaxed_nmadd))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("f64x2.relaxed_nmadd"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn f64x2_relaxed_nmadd (a : v128 , b : v128 , c : v128) -> v128 { unsafe { llvm_f64x2_fms (a . as_f64x2 () , b . as_f64x2 () , c . as_f64x2 ()) . v128 () } }
}

macro_rules! i8x16_relaxed_laneselect_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i8x16_relaxed_laneselect in module {}", module_path!());
    };
}

mkfn!{
    i8x16_relaxed_laneselect_introspect!();
    # [doc = " A relaxed version of `v128_bitselect` where this either behaves the same as"] # [doc = " `v128_bitselect` or the high bit of each lane `m` is inspected and the"] # [doc = " corresponding lane of `a` is chosen if the bit is 1 or the lane of `b` is"] # [doc = " chosen if it's zero."] # [doc = ""] # [doc = " If the `m` mask's lanes are either all-one or all-zero then this instruction"] # [doc = " is the same as `v128_bitselect`."] # [inline] # [cfg_attr (test , assert_instr (i8x16 . relaxed_laneselect))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i8x16.relaxed_laneselect"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn i8x16_relaxed_laneselect (a : v128 , b : v128 , m : v128) -> v128 { unsafe { llvm_i8x16_laneselect (a . as_i8x16 () , b . as_i8x16 () , m . as_i8x16 ()) . v128 () } }
}
mkuse!{# [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub use i8x16_relaxed_laneselect as u8x16_relaxed_laneselect ;}

macro_rules! i16x8_relaxed_laneselect_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i16x8_relaxed_laneselect in module {}", module_path!());
    };
}

mkfn!{
    i16x8_relaxed_laneselect_introspect!();
    # [doc = " A relaxed version of `v128_bitselect` where this either behaves the same as"] # [doc = " `v128_bitselect` or the high bit of each lane `m` is inspected and the"] # [doc = " corresponding lane of `a` is chosen if the bit is 1 or the lane of `b` is"] # [doc = " chosen if it's zero."] # [doc = ""] # [doc = " If the `m` mask's lanes are either all-one or all-zero then this instruction"] # [doc = " is the same as `v128_bitselect`."] # [inline] # [cfg_attr (test , assert_instr (i16x8 . relaxed_laneselect))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i16x8.relaxed_laneselect"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn i16x8_relaxed_laneselect (a : v128 , b : v128 , m : v128) -> v128 { unsafe { llvm_i16x8_laneselect (a . as_i16x8 () , b . as_i16x8 () , m . as_i16x8 ()) . v128 () } }
}
mkuse!{# [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub use i16x8_relaxed_laneselect as u16x8_relaxed_laneselect ;}

macro_rules! i32x4_relaxed_laneselect_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i32x4_relaxed_laneselect in module {}", module_path!());
    };
}

mkfn!{
    i32x4_relaxed_laneselect_introspect!();
    # [doc = " A relaxed version of `v128_bitselect` where this either behaves the same as"] # [doc = " `v128_bitselect` or the high bit of each lane `m` is inspected and the"] # [doc = " corresponding lane of `a` is chosen if the bit is 1 or the lane of `b` is"] # [doc = " chosen if it's zero."] # [doc = ""] # [doc = " If the `m` mask's lanes are either all-one or all-zero then this instruction"] # [doc = " is the same as `v128_bitselect`."] # [inline] # [cfg_attr (test , assert_instr (i32x4 . relaxed_laneselect))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i32x4.relaxed_laneselect"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn i32x4_relaxed_laneselect (a : v128 , b : v128 , m : v128) -> v128 { unsafe { llvm_i32x4_laneselect (a . as_i32x4 () , b . as_i32x4 () , m . as_i32x4 ()) . v128 () } }
}
mkuse!{# [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub use i32x4_relaxed_laneselect as u32x4_relaxed_laneselect ;}

macro_rules! i64x2_relaxed_laneselect_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i64x2_relaxed_laneselect in module {}", module_path!());
    };
}

mkfn!{
    i64x2_relaxed_laneselect_introspect!();
    # [doc = " A relaxed version of `v128_bitselect` where this either behaves the same as"] # [doc = " `v128_bitselect` or the high bit of each lane `m` is inspected and the"] # [doc = " corresponding lane of `a` is chosen if the bit is 1 or the lane of `b` is"] # [doc = " chosen if it's zero."] # [doc = ""] # [doc = " If the `m` mask's lanes are either all-one or all-zero then this instruction"] # [doc = " is the same as `v128_bitselect`."] # [inline] # [cfg_attr (test , assert_instr (i64x2 . relaxed_laneselect))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i64x2.relaxed_laneselect"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn i64x2_relaxed_laneselect (a : v128 , b : v128 , m : v128) -> v128 { unsafe { llvm_i64x2_laneselect (a . as_i64x2 () , b . as_i64x2 () , m . as_i64x2 ()) . v128 () } }
}
mkuse!{# [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub use i64x2_relaxed_laneselect as u64x2_relaxed_laneselect ;}

macro_rules! f32x4_relaxed_min_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f32x4_relaxed_min in module {}", module_path!());
    };
}

mkfn!{
    f32x4_relaxed_min_introspect!();
    # [doc = " A relaxed version of `f32x4_min` which is either `f32x4_min` or"] # [doc = " `f32x4_pmin`."] # [inline] # [cfg_attr (test , assert_instr (f32x4 . relaxed_min))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("f32x4.relaxed_min"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn f32x4_relaxed_min (a : v128 , b : v128) -> v128 { unsafe { llvm_f32x4_relaxed_min (a . as_f32x4 () , b . as_f32x4 ()) . v128 () } }
}

macro_rules! f32x4_relaxed_max_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f32x4_relaxed_max in module {}", module_path!());
    };
}

mkfn!{
    f32x4_relaxed_max_introspect!();
    # [doc = " A relaxed version of `f32x4_max` which is either `f32x4_max` or"] # [doc = " `f32x4_pmax`."] # [inline] # [cfg_attr (test , assert_instr (f32x4 . relaxed_max))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("f32x4.relaxed_max"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn f32x4_relaxed_max (a : v128 , b : v128) -> v128 { unsafe { llvm_f32x4_relaxed_max (a . as_f32x4 () , b . as_f32x4 ()) . v128 () } }
}

macro_rules! f64x2_relaxed_min_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f64x2_relaxed_min in module {}", module_path!());
    };
}

mkfn!{
    f64x2_relaxed_min_introspect!();
    # [doc = " A relaxed version of `f64x2_min` which is either `f64x2_min` or"] # [doc = " `f64x2_pmin`."] # [inline] # [cfg_attr (test , assert_instr (f64x2 . relaxed_min))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("f64x2.relaxed_min"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn f64x2_relaxed_min (a : v128 , b : v128) -> v128 { unsafe { llvm_f64x2_relaxed_min (a . as_f64x2 () , b . as_f64x2 ()) . v128 () } }
}

macro_rules! f64x2_relaxed_max_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f64x2_relaxed_max in module {}", module_path!());
    };
}

mkfn!{
    f64x2_relaxed_max_introspect!();
    # [doc = " A relaxed version of `f64x2_max` which is either `f64x2_max` or"] # [doc = " `f64x2_pmax`."] # [inline] # [cfg_attr (test , assert_instr (f64x2 . relaxed_max))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("f64x2.relaxed_max"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn f64x2_relaxed_max (a : v128 , b : v128) -> v128 { unsafe { llvm_f64x2_relaxed_max (a . as_f64x2 () , b . as_f64x2 ()) . v128 () } }
}

macro_rules! i16x8_relaxed_q15mulr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i16x8_relaxed_q15mulr in module {}", module_path!());
    };
}

mkfn!{
    i16x8_relaxed_q15mulr_introspect!();
    # [doc = " A relaxed version of `i16x8_relaxed_q15mulr` where if both lanes are"] # [doc = " `i16::MIN` then the result is either `i16::MIN` or `i16::MAX`."] # [inline] # [cfg_attr (test , assert_instr (i16x8 . relaxed_q15mulr_s))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i16x8.relaxed_q15mulr_s"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn i16x8_relaxed_q15mulr (a : v128 , b : v128) -> v128 { unsafe { llvm_relaxed_q15mulr_signed (a . as_i16x8 () , b . as_i16x8 ()) . v128 () } }
}
mkuse!{# [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub use i16x8_relaxed_q15mulr as u16x8_relaxed_q15mulr ;}

macro_rules! i16x8_relaxed_dot_i8x16_i7x16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i16x8_relaxed_dot_i8x16_i7x16 in module {}", module_path!());
    };
}

mkfn!{
    i16x8_relaxed_dot_i8x16_i7x16_introspect!();
    # [doc = " A relaxed dot-product instruction."] # [doc = ""] # [doc = " This instruction will perform pairwise products of the 8-bit values in `a`"] # [doc = " and `b` and then accumulate adjacent pairs into 16-bit results producing a"] # [doc = " final `i16x8` vector. The bytes of `a` are always interpreted as signed and"] # [doc = " the bytes in `b` may be interpreted as signed or unsigned. If the top bit in"] # [doc = " `b` isn't set then the value is the same regardless of whether it's signed"] # [doc = " or unsigned."] # [doc = ""] # [doc = " The accumulation into 16-bit values may be saturated on some platforms, and"] # [doc = " on other platforms it may wrap-around on overflow."] # [inline] # [cfg_attr (test , assert_instr (i16x8 . relaxed_dot_i8x16_i7x16_s))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i16x8.relaxed_dot_i8x16_i7x16_s"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn i16x8_relaxed_dot_i8x16_i7x16 (a : v128 , b : v128) -> v128 { unsafe { llvm_i16x8_relaxed_dot_i8x16_i7x16_s (a . as_i8x16 () , b . as_i8x16 ()) . v128 () } }
}
mkuse!{# [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub use i16x8_relaxed_dot_i8x16_i7x16 as u16x8_relaxed_dot_i8x16_i7x16 ;}

macro_rules! i32x4_relaxed_dot_i8x16_i7x16_add_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i32x4_relaxed_dot_i8x16_i7x16_add in module {}", module_path!());
    };
}

mkfn!{
    i32x4_relaxed_dot_i8x16_i7x16_add_introspect!();
    # [doc = " Similar to [`i16x8_relaxed_dot_i8x16_i7x16`] except that the intermediate"] # [doc = " `i16x8` result is fed into `i32x4_extadd_pairwise_i16x8` followed by"] # [doc = " `i32x4_add` to add the value `c` to the result."] # [inline] # [cfg_attr (test , assert_instr (i32x4 . relaxed_dot_i8x16_i7x16_add_s))] # [target_feature (enable = "relaxed-simd")] # [doc (alias ("i32x4.relaxed_dot_i8x16_i7x16_add_s"))] # [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub fn i32x4_relaxed_dot_i8x16_i7x16_add (a : v128 , b : v128 , c : v128) -> v128 { unsafe { llvm_i32x4_relaxed_dot_i8x16_i7x16_add_s (a . as_i8x16 () , b . as_i8x16 () , c . as_i32x4 ()) . v128 () } }
}
mkuse!{# [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub use i32x4_relaxed_dot_i8x16_i7x16_add as u32x4_relaxed_dot_i8x16_i7x16_add ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: super :: simd128 :: * ;}
mkuse!{use super :: * ;}
mkuse!{use core :: ops :: { Add , Div , Mul , Neg , Sub } ;}
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: mem :: transmute ;}
mkuse!{use std :: num :: Wrapping ;}
mkuse!{use std :: prelude :: v1 :: * ;}

macro_rules! compare_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compare_bytes in module {}", module_path!());
    };
}

mkfn!{
    compare_bytes_introspect!();
    fn compare_bytes (a : v128 , b : & [v128]) { let a : [u8 ; 16] = unsafe { transmute (a) } ; if b . iter () . any (| b | { let b : [u8 ; 16] = unsafe { transmute (* b) } ; a == b }) { return ; } eprintln ! ("input vector {a:?}") ; eprintln ! ("did not match any output:") ; for b in b { eprintln ! ("  {b:?}") ; } }
}

macro_rules! test_relaxed_swizzle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_relaxed_swizzle in module {}", module_path!());
    };
}

mkfn!{
    test_relaxed_swizzle_introspect!();
    # [test] fn test_relaxed_swizzle () { compare_bytes (i8x16_relaxed_swizzle (i8x16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) , i8x16 (2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 0 , 1) ,) , & [i8x16 (2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 0 , 1)] ,) ; compare_bytes (i8x16_relaxed_swizzle (i8x16 (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) , u8x16 (0x80 , 0xff , 16 , 17 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ,) , & [i8x16 (0 , 0 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) , i8x16 (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ,] ,) ; compare_bytes (u8x16_relaxed_swizzle (u8x16 (128 , 129 , 130 , 131 , 132 , 133 , 134 , 135 , 136 , 137 , 138 , 139 , 140 , 141 , 142 , 143 ,) , u8x16 (0x80 , 0xff , 16 , 17 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0) ,) , & [u8x16 (128 , 128 , 128 , 129 , 128 , 128 , 128 , 128 , 128 , 128 , 128 , 128 , 128 , 128 , 128 , 128 ,) , u8x16 (0 , 0 , 0 , 0 , 128 , 128 , 128 , 128 , 128 , 128 , 128 , 128 , 128 , 128 , 128 , 128 ,) ,] ,) ; }
}

macro_rules! test_relaxed_trunc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_relaxed_trunc in module {}", module_path!());
    };
}

mkfn!{
    test_relaxed_trunc_introspect!();
    # [test] fn test_relaxed_trunc () { compare_bytes (i32x4_relaxed_trunc_f32x4 (f32x4 (1.0 , 2.0 , - 1. , - 4.)) , & [i32x4 (1 , 2 , - 1 , - 4)] ,) ; compare_bytes (i32x4_relaxed_trunc_f32x4 (f32x4 (f32 :: NEG_INFINITY , f32 :: NAN , - 0.0 , f32 :: INFINITY)) , & [i32x4 (i32 :: MIN , 0 , 0 , i32 :: MAX) , i32x4 (i32 :: MIN , i32 :: MIN , 0 , i32 :: MIN) ,] ,) ; compare_bytes (i32x4_relaxed_trunc_f64x2_zero (f64x2 (1.0 , - 3.0)) , & [i32x4 (1 , - 3 , 0 , 0)] ,) ; compare_bytes (i32x4_relaxed_trunc_f64x2_zero (f64x2 (f64 :: INFINITY , f64 :: NAN)) , & [i32x4 (i32 :: MAX , 0 , 0 , 0) , i32x4 (i32 :: MIN , i32 :: MIN , 0 , 0)] ,) ; compare_bytes (u32x4_relaxed_trunc_f32x4 (f32x4 (1.0 , 2.0 , 5. , 100.)) , & [i32x4 (1 , 2 , 5 , 100)] ,) ; compare_bytes (u32x4_relaxed_trunc_f32x4 (f32x4 (f32 :: NEG_INFINITY , f32 :: NAN , - 0.0 , f32 :: INFINITY)) , & [u32x4 (u32 :: MAX , 0 , 0 , u32 :: MAX) , u32x4 (u32 :: MAX , u32 :: MAX , 0 , u32 :: MAX) ,] ,) ; compare_bytes (u32x4_relaxed_trunc_f64x2_zero (f64x2 (1.0 , 3.0)) , & [u32x4 (1 , 3 , 0 , 0)] ,) ; compare_bytes (u32x4_relaxed_trunc_f64x2_zero (f64x2 (f64 :: INFINITY , f64 :: NAN)) , & [i32x4 (i32 :: MAX , 0 , 0 , 0) , i32x4 (i32 :: MIN , i32 :: MIN , 0 , 0)] ,) ; }
}

macro_rules! test_madd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_madd in module {}", module_path!());
    };
}

mkfn!{
    test_madd_introspect!();
    # [test] fn test_madd () { let floats = [f32 :: NAN , f32 :: NEG_INFINITY , f32 :: INFINITY , 1.0 , 2.0 , - 1.0 , 0.0 , 100.3 , 7.8 , 9.4 ,] ; for & a in floats . iter () { for & b in floats . iter () { for & c in floats . iter () { let f1 = a * b + c ; let f2 = a . mul_add (b , c) ; compare_bytes (f32x4_relaxed_madd (f32x4 (a , a , a , a) , f32x4 (b , b , b , b) , f32x4 (c , c , c , c)) , & [f32x4 (f1 , f1 , f1 , f1) , f32x4 (f2 , f2 , f2 , f2)] ,) ; let f1 = - a * b + c ; let f2 = (- a) . mul_add (b , c) ; compare_bytes (f32x4_relaxed_nmadd (f32x4 (a , a , a , a) , f32x4 (b , b , b , b) , f32x4 (c , c , c , c) ,) , & [f32x4 (f1 , f1 , f1 , f1) , f32x4 (f2 , f2 , f2 , f2)] ,) ; let a = f64 :: from (a) ; let b = f64 :: from (b) ; let c = f64 :: from (c) ; let f1 = a * b + c ; let f2 = a . mul_add (b , c) ; compare_bytes (f64x2_relaxed_madd (f64x2 (a , a) , f64x2 (b , b) , f64x2 (c , c)) , & [f64x2 (f1 , f1) , f64x2 (f2 , f2)] ,) ; let f1 = - a * b + c ; let f2 = (- a) . mul_add (b , c) ; compare_bytes (f64x2_relaxed_nmadd (f64x2 (a , a) , f64x2 (b , b) , f64x2 (c , c)) , & [f64x2 (f1 , f1) , f64x2 (f2 , f2)] ,) ; } } } }
} 
            }}