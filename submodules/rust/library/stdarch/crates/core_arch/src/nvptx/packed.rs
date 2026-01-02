mkuse!{use crate :: intrinsics :: simd :: * ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.minimum.v2f16"] fn llvm_f16x2_minimum (a : f16x2 , b : f16x2) -> f16x2 ; # [link_name = "llvm.maximum.v2f16"] fn llvm_f16x2_maximum (a : f16x2 , b : f16x2) -> f16x2 ; }}
mkitem!{types ! { #! [unstable (feature = "stdarch_nvptx" , issue = "111199")] # [doc = " PTX-specific 32-bit wide floating point (f16 x 2) vector type"] pub struct f16x2 (2 x f16) ; }}

macro_rules! f16x2_add_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f16x2_add in module {}", module_path!());
    };
}

mkfn!{
    f16x2_add_introspect!();
    # [doc = " Add two values, round to nearest even"] # [doc = ""] # [doc = " <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-add>"] # [doc = ""] # [doc = " Corresponds to the CUDA C intrinsics:"] # [doc = "  - [`__hadd2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g921c795176eaa31265bd80ef4fe4b8e6)"] # [doc = "  - [`__hadd2_rn`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g6cd8ddb2c3d670e1a10c3eb2e7644f82)"] # [inline] # [cfg_attr (test , assert_instr (add . rn . f16x22))] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn f16x2_add (a : f16x2 , b : f16x2) -> f16x2 { simd_add (a , b) }
}

macro_rules! f16x2_sub_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f16x2_sub in module {}", module_path!());
    };
}

mkfn!{
    f16x2_sub_introspect!();
    # [doc = " Subtract two values, round to nearest even"] # [doc = ""] # [doc = " <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-sub>"] # [doc = ""] # [doc = " Corresponds to the CUDA C intrinsics:"] # [doc = "  - [`__hsub2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1ga5536c9c3d853d8c8b9de60e18b41e54)"] # [doc = "  - [`__hsub2_rn`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g8adc164c68d553354f749f0f0645a874)"] # [inline] # [cfg_attr (test , assert_instr (sub . rn . f16x2))] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn f16x2_sub (a : f16x2 , b : f16x2) -> f16x2 { simd_sub (a , b) }
}

macro_rules! f16x2_mul_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f16x2_mul in module {}", module_path!());
    };
}

mkfn!{
    f16x2_mul_introspect!();
    # [doc = " Multiply two values, round to nearest even"] # [doc = ""] # [doc = " <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-mul>"] # [doc = ""] # [doc = " Corresponds to the CUDA C intrinsics:"] # [doc = "  - [`__hmul2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g70de3f2ee48babe4e0969397ac17708e)"] # [doc = "  - [`__hmul2_rn`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g99f8fe23a4b4c6898d6faf999afaa76e)"] # [inline] # [cfg_attr (test , assert_instr (mul . rn . f16x2))] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn f16x2_mul (a : f16x2 , b : f16x2) -> f16x2 { simd_mul (a , b) }
}

macro_rules! f16x2_fma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f16x2_fma in module {}", module_path!());
    };
}

mkfn!{
    f16x2_fma_introspect!();
    # [doc = " Fused multiply-add, round to nearest even"] # [doc = ""] # [doc = " <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-fma>"] # [doc = ""] # [doc = " Corresponds to the CUDA C intrinsics:"] # [doc = "  - [`__fma2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g43628ba21ded8b1e188a367348008dab)"] # [doc = "  - [`__fma2_rn`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g43628ba21ded8b1e188a367348008dab)"] # [inline] # [cfg_attr (test , assert_instr (fma . rn . f16x2))] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn f16x2_fma (a : f16x2 , b : f16x2 , c : f16x2) -> f16x2 { simd_fma (a , b , c) }
}

macro_rules! f16x2_neg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f16x2_neg in module {}", module_path!());
    };
}

mkfn!{
    f16x2_neg_introspect!();
    # [doc = " Arithmetic negate"] # [doc = ""] # [doc = " <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-neg>"] # [doc = ""] # [doc = " Corresponds to the CUDA C intrinsic [`__hmin2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__COMPARISON.html#group__CUDA__MATH____HALF2__COMPARISON_1g9e17a33f96061804166f3fbd395422b6)"] # [inline] # [cfg_attr (test , assert_instr (neg . f16x2))] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn f16x2_neg (a : f16x2) -> f16x2 { simd_neg (a) }
}

macro_rules! f16x2_min_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f16x2_min in module {}", module_path!());
    };
}

mkfn!{
    f16x2_min_introspect!();
    # [doc = " Find the minimum of two values"] # [doc = ""] # [doc = " <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-min>"] # [doc = ""] # [doc = " Corresponds to the CUDA C intrinsic [`__hmin2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__COMPARISON.html#group__CUDA__MATH____HALF2__COMPARISON_1g9e17a33f96061804166f3fbd395422b6)"] # [inline] # [cfg_attr (test , assert_instr (min . f16x2))] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn f16x2_min (a : f16x2 , b : f16x2) -> f16x2 { simd_fmin (a , b) }
}

macro_rules! f16x2_min_nan_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f16x2_min_nan in module {}", module_path!());
    };
}

mkfn!{
    f16x2_min_nan_introspect!();
    # [doc = " Find the minimum of two values, NaNs pass through."] # [doc = ""] # [doc = " <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-min>"] # [doc = ""] # [doc = " Corresponds to the CUDA C intrinsic [`__hmin2_nan`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__COMPARISON.html#group__CUDA__MATH____HALF2__COMPARISON_1g8bb8f58e9294cc261d2f42c4d5aecd6b)"] # [inline] # [cfg_attr (test , assert_instr (min . NaN . f16x2))] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn f16x2_min_nan (a : f16x2 , b : f16x2) -> f16x2 { llvm_f16x2_minimum (a , b) }
}

macro_rules! f16x2_max_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f16x2_max in module {}", module_path!());
    };
}

mkfn!{
    f16x2_max_introspect!();
    # [doc = " Find the maximum of two values"] # [doc = ""] # [doc = " <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-max>"] # [doc = ""] # [doc = " Corresponds to the CUDA C intrinsic [`__hmax2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__COMPARISON.html#group__CUDA__MATH____HALF2__COMPARISON_1g59fc7fc7975d8127b202444a05e57e3d)"] # [inline] # [cfg_attr (test , assert_instr (max . f16x2))] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn f16x2_max (a : f16x2 , b : f16x2) -> f16x2 { simd_fmax (a , b) }
}

macro_rules! f16x2_max_nan_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f16x2_max_nan in module {}", module_path!());
    };
}

mkfn!{
    f16x2_max_nan_introspect!();
    # [doc = " Find the maximum of two values, NaNs pass through."] # [doc = ""] # [doc = " <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-max>"] # [doc = ""] # [doc = " Corresponds to the CUDA C intrinsic [`__hmax2_nan`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__COMPARISON.html#group__CUDA__MATH____HALF2__COMPARISON_1g41623db7850e3074fd9daa80a14c3897)"] # [inline] # [cfg_attr (test , assert_instr (max . NaN . f16x2))] # [unstable (feature = "stdarch_nvptx" , issue = "111199")] pub unsafe fn f16x2_max_nan (a : f16x2 , b : f16x2) -> f16x2 { llvm_f16x2_maximum (a , b) }
}