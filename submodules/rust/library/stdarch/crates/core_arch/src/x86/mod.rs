mkuse!{use crate :: mem :: transmute ;}
mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkitem!{types ! { #! [stable (feature = "simd_x86" , since = "1.27.0")] # [doc = " 128-bit wide integer vector type, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m128i` type defined by Intel,"] # [doc = " representing a 128-bit SIMD register. Usage of this type typically"] # [doc = " corresponds to the `sse` and up target features for x86/x86_64."] # [doc = ""] # [doc = " Internally this type may be viewed as:"] # [doc = ""] # [doc = " * `i8x16` - sixteen `i8` variables packed together"] # [doc = " * `i16x8` - eight `i16` variables packed together"] # [doc = " * `i32x4` - four `i32` variables packed together"] # [doc = " * `i64x2` - two `i64` variables packed together"] # [doc = ""] # [doc = " (as well as unsigned versions). Each intrinsic may interpret the"] # [doc = " internal bits differently, check the documentation of the intrinsic"] # [doc = " to see how it's being used."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] # [doc = ""] # [doc = " Note that this means that an instance of `__m128i` typically just means"] # [doc = " a \"bag of bits\" which is left up to interpretation at the point of use."] # [doc = ""] # [doc = " Most intrinsics using `__m128i` are prefixed with `_mm_` and the"] # [doc = " integer types tend to correspond to suffixes like \"epi8\" or \"epi32\"."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #[cfg(target_arch = \"x86\")]"] # [doc = " use std::arch::x86::*;"] # [doc = " #[cfg(target_arch = \"x86_64\")]"] # [doc = " use std::arch::x86_64::*;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " # #[target_feature(enable = \"sse2\")]"] # [doc = " # #[allow(unused_unsafe)] // temporary, to unstick CI"] # [doc = " # unsafe fn foo() { unsafe {"] # [doc = " let all_bytes_zero = _mm_setzero_si128();"] # [doc = " let all_bytes_one = _mm_set1_epi8(1);"] # [doc = " let four_i32 = _mm_set_epi32(1, 2, 3, 4);"] # [doc = " # }}"] # [doc = " # if is_x86_feature_detected!(\"sse2\") { unsafe { foo() } }"] # [doc = " # }"] # [doc = " ```"] pub struct __m128i (2 x i64) ; # [doc = " 128-bit wide set of four `f32` types, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m128` type defined by Intel,"] # [doc = " representing a 128-bit SIMD register which internally is consisted of"] # [doc = " four packed `f32` instances. Usage of this type typically corresponds"] # [doc = " to the `sse` and up target features for x86/x86_64."] # [doc = ""] # [doc = " Note that unlike `__m128i`, the integer version of the 128-bit"] # [doc = " registers, this `__m128` type has *one* interpretation. Each instance"] # [doc = " of `__m128` always corresponds to `f32x4`, or four `f32` types packed"] # [doc = " together."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] # [doc = ""] # [doc = " Most intrinsics using `__m128` are prefixed with `_mm_` and are"] # [doc = " suffixed with \"ps\" (or otherwise contain \"ps\"). Not to be confused with"] # [doc = " \"pd\" which is used for `__m128d`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #[cfg(target_arch = \"x86\")]"] # [doc = " use std::arch::x86::*;"] # [doc = " #[cfg(target_arch = \"x86_64\")]"] # [doc = " use std::arch::x86_64::*;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " # #[target_feature(enable = \"sse\")]"] # [doc = " # #[allow(unused_unsafe)] // temporary, to unstick CI"] # [doc = " # unsafe fn foo() { unsafe {"] # [doc = " let four_zeros = _mm_setzero_ps();"] # [doc = " let four_ones = _mm_set1_ps(1.0);"] # [doc = " let four_floats = _mm_set_ps(1.0, 2.0, 3.0, 4.0);"] # [doc = " # }}"] # [doc = " # if is_x86_feature_detected!(\"sse\") { unsafe { foo() } }"] # [doc = " # }"] # [doc = " ```"] pub struct __m128 (4 x f32) ; # [doc = " 128-bit wide set of two `f64` types, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m128d` type defined by Intel,"] # [doc = " representing a 128-bit SIMD register which internally is consisted of"] # [doc = " two packed `f64` instances. Usage of this type typically corresponds"] # [doc = " to the `sse` and up target features for x86/x86_64."] # [doc = ""] # [doc = " Note that unlike `__m128i`, the integer version of the 128-bit"] # [doc = " registers, this `__m128d` type has *one* interpretation. Each instance"] # [doc = " of `__m128d` always corresponds to `f64x2`, or two `f64` types packed"] # [doc = " together."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] # [doc = ""] # [doc = " Most intrinsics using `__m128d` are prefixed with `_mm_` and are"] # [doc = " suffixed with \"pd\" (or otherwise contain \"pd\"). Not to be confused with"] # [doc = " \"ps\" which is used for `__m128`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #[cfg(target_arch = \"x86\")]"] # [doc = " use std::arch::x86::*;"] # [doc = " #[cfg(target_arch = \"x86_64\")]"] # [doc = " use std::arch::x86_64::*;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " # #[target_feature(enable = \"sse2\")]"] # [doc = " # #[allow(unused_unsafe)] // temporary, to unstick CI"] # [doc = " # unsafe fn foo() { unsafe {"] # [doc = " let two_zeros = _mm_setzero_pd();"] # [doc = " let two_ones = _mm_set1_pd(1.0);"] # [doc = " let two_floats = _mm_set_pd(1.0, 2.0);"] # [doc = " # }}"] # [doc = " # if is_x86_feature_detected!(\"sse2\") { unsafe { foo() } }"] # [doc = " # }"] # [doc = " ```"] pub struct __m128d (2 x f64) ; # [doc = " 256-bit wide integer vector type, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m256i` type defined by Intel,"] # [doc = " representing a 256-bit SIMD register. Usage of this type typically"] # [doc = " corresponds to the `avx` and up target features for x86/x86_64."] # [doc = ""] # [doc = " Internally this type may be viewed as:"] # [doc = ""] # [doc = " * `i8x32` - thirty two `i8` variables packed together"] # [doc = " * `i16x16` - sixteen `i16` variables packed together"] # [doc = " * `i32x8` - eight `i32` variables packed together"] # [doc = " * `i64x4` - four `i64` variables packed together"] # [doc = ""] # [doc = " (as well as unsigned versions). Each intrinsic may interpret the"] # [doc = " internal bits differently, check the documentation of the intrinsic"] # [doc = " to see how it's being used."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] # [doc = ""] # [doc = " Note that this means that an instance of `__m256i` typically just means"] # [doc = " a \"bag of bits\" which is left up to interpretation at the point of use."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #[cfg(target_arch = \"x86\")]"] # [doc = " use std::arch::x86::*;"] # [doc = " #[cfg(target_arch = \"x86_64\")]"] # [doc = " use std::arch::x86_64::*;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " # #[target_feature(enable = \"avx\")]"] # [doc = " # #[allow(unused_unsafe)] // temporary, to unstick CI"] # [doc = " # unsafe fn foo() { unsafe {"] # [doc = " let all_bytes_zero = _mm256_setzero_si256();"] # [doc = " let all_bytes_one = _mm256_set1_epi8(1);"] # [doc = " let eight_i32 = _mm256_set_epi32(1, 2, 3, 4, 5, 6, 7, 8);"] # [doc = " # }}"] # [doc = " # if is_x86_feature_detected!(\"avx\") { unsafe { foo() } }"] # [doc = " # }"] # [doc = " ```"] pub struct __m256i (4 x i64) ; # [doc = " 256-bit wide set of eight `f32` types, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m256` type defined by Intel,"] # [doc = " representing a 256-bit SIMD register which internally is consisted of"] # [doc = " eight packed `f32` instances. Usage of this type typically corresponds"] # [doc = " to the `avx` and up target features for x86/x86_64."] # [doc = ""] # [doc = " Note that unlike `__m256i`, the integer version of the 256-bit"] # [doc = " registers, this `__m256` type has *one* interpretation. Each instance"] # [doc = " of `__m256` always corresponds to `f32x8`, or eight `f32` types packed"] # [doc = " together."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding  between two consecutive elements); however, the"] # [doc = " alignment is different and equal to the size of the type. Note that the"] # [doc = " ABI for function calls may *not* be the same."] # [doc = ""] # [doc = " Most intrinsics using `__m256` are prefixed with `_mm256_` and are"] # [doc = " suffixed with \"ps\" (or otherwise contain \"ps\"). Not to be confused with"] # [doc = " \"pd\" which is used for `__m256d`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #[cfg(target_arch = \"x86\")]"] # [doc = " use std::arch::x86::*;"] # [doc = " #[cfg(target_arch = \"x86_64\")]"] # [doc = " use std::arch::x86_64::*;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " # #[target_feature(enable = \"avx\")]"] # [doc = " # #[allow(unused_unsafe)] // temporary, to unstick CI"] # [doc = " # unsafe fn foo() { unsafe {"] # [doc = " let eight_zeros = _mm256_setzero_ps();"] # [doc = " let eight_ones = _mm256_set1_ps(1.0);"] # [doc = " let eight_floats = _mm256_set_ps(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);"] # [doc = " # }}"] # [doc = " # if is_x86_feature_detected!(\"avx\") { unsafe { foo() } }"] # [doc = " # }"] # [doc = " ```"] pub struct __m256 (8 x f32) ; # [doc = " 256-bit wide set of four `f64` types, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m256d` type defined by Intel,"] # [doc = " representing a 256-bit SIMD register which internally is consisted of"] # [doc = " four packed `f64` instances. Usage of this type typically corresponds"] # [doc = " to the `avx` and up target features for x86/x86_64."] # [doc = ""] # [doc = " Note that unlike `__m256i`, the integer version of the 256-bit"] # [doc = " registers, this `__m256d` type has *one* interpretation. Each instance"] # [doc = " of `__m256d` always corresponds to `f64x4`, or four `f64` types packed"] # [doc = " together."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] # [doc = ""] # [doc = " Most intrinsics using `__m256d` are prefixed with `_mm256_` and are"] # [doc = " suffixed with \"pd\" (or otherwise contain \"pd\"). Not to be confused with"] # [doc = " \"ps\" which is used for `__m256`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #[cfg(target_arch = \"x86\")]"] # [doc = " use std::arch::x86::*;"] # [doc = " #[cfg(target_arch = \"x86_64\")]"] # [doc = " use std::arch::x86_64::*;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " # #[target_feature(enable = \"avx\")]"] # [doc = " # #[allow(unused_unsafe)] // temporary, to unstick CI"] # [doc = " # unsafe fn foo() { unsafe {"] # [doc = " let four_zeros = _mm256_setzero_pd();"] # [doc = " let four_ones = _mm256_set1_pd(1.0);"] # [doc = " let four_floats = _mm256_set_pd(1.0, 2.0, 3.0, 4.0);"] # [doc = " # }}"] # [doc = " # if is_x86_feature_detected!(\"avx\") { unsafe { foo() } }"] # [doc = " # }"] # [doc = " ```"] pub struct __m256d (4 x f64) ; }}
mkitem!{types ! { #! [stable (feature = "simd_avx512_types" , since = "1.72.0")] # [doc = " 512-bit wide integer vector type, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m512i` type defined by Intel,"] # [doc = " representing a 512-bit SIMD register. Usage of this type typically"] # [doc = " corresponds to the `avx512*` and up target features for x86/x86_64."] # [doc = ""] # [doc = " Internally this type may be viewed as:"] # [doc = ""] # [doc = " * `i8x64` - sixty-four `i8` variables packed together"] # [doc = " * `i16x32` - thirty-two `i16` variables packed together"] # [doc = " * `i32x16` - sixteen `i32` variables packed together"] # [doc = " * `i64x8` - eight `i64` variables packed together"] # [doc = ""] # [doc = " (as well as unsigned versions). Each intrinsic may interpret the"] # [doc = " internal bits differently, check the documentation of the intrinsic"] # [doc = " to see how it's being used."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] # [doc = ""] # [doc = " Note that this means that an instance of `__m512i` typically just means"] # [doc = " a \"bag of bits\" which is left up to interpretation at the point of use."] pub struct __m512i (8 x i64) ; # [doc = " 512-bit wide set of sixteen `f32` types, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m512` type defined by Intel,"] # [doc = " representing a 512-bit SIMD register which internally is consisted of"] # [doc = " eight packed `f32` instances. Usage of this type typically corresponds"] # [doc = " to the `avx512*` and up target features for x86/x86_64."] # [doc = ""] # [doc = " Note that unlike `__m512i`, the integer version of the 512-bit"] # [doc = " registers, this `__m512` type has *one* interpretation. Each instance"] # [doc = " of `__m512` always corresponds to `f32x16`, or sixteen `f32` types"] # [doc = " packed together."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding  between two consecutive elements); however, the"] # [doc = " alignment is different and equal to the size of the type. Note that the"] # [doc = " ABI for function calls may *not* be the same."] # [doc = ""] # [doc = " Most intrinsics using `__m512` are prefixed with `_mm512_` and are"] # [doc = " suffixed with \"ps\" (or otherwise contain \"ps\"). Not to be confused with"] # [doc = " \"pd\" which is used for `__m512d`."] pub struct __m512 (16 x f32) ; # [doc = " 512-bit wide set of eight `f64` types, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m512d` type defined by Intel,"] # [doc = " representing a 512-bit SIMD register which internally is consisted of"] # [doc = " eight packed `f64` instances. Usage of this type typically corresponds"] # [doc = " to the `avx` and up target features for x86/x86_64."] # [doc = ""] # [doc = " Note that unlike `__m512i`, the integer version of the 512-bit"] # [doc = " registers, this `__m512d` type has *one* interpretation. Each instance"] # [doc = " of `__m512d` always corresponds to `f64x8`, or eight `f64` types packed"] # [doc = " together."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding  between two consecutive elements); however, the"] # [doc = " alignment is different and equal to the size of the type. Note that the"] # [doc = " ABI for function calls may *not* be the same."] # [doc = ""] # [doc = " Most intrinsics using `__m512d` are prefixed with `_mm512_` and are"] # [doc = " suffixed with \"pd\" (or otherwise contain \"pd\"). Not to be confused with"] # [doc = " \"ps\" which is used for `__m512`."] pub struct __m512d (8 x f64) ; }}
mkitem!{types ! { #! [stable (feature = "stdarch_x86_avx512" , since = "1.89")] # [doc = " 128-bit wide set of eight `u16` types, x86-specific"] # [doc = ""] # [doc = " This type is representing a 128-bit SIMD register which internally is consisted of"] # [doc = " eight packed `u16` instances. Its purpose is for bf16 related intrinsic"] # [doc = " implementations."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] pub struct __m128bh (8 x u16) ; # [doc = " 256-bit wide set of 16 `u16` types, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m256bh` type defined by Intel,"] # [doc = " representing a 256-bit SIMD register which internally is consisted of"] # [doc = " 16 packed `u16` instances. Its purpose is for bf16 related intrinsic"] # [doc = " implementations."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] pub struct __m256bh (16 x u16) ; # [doc = " 512-bit wide set of 32 `u16` types, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m512bh` type defined by Intel,"] # [doc = " representing a 512-bit SIMD register which internally is consisted of"] # [doc = " 32 packed `u16` instances. Its purpose is for bf16 related intrinsic"] # [doc = " implementations."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] pub struct __m512bh (32 x u16) ; }}
mkitem!{types ! { #! [unstable (feature = "stdarch_x86_avx512_f16" , issue = "127213")] # [doc = " 128-bit wide set of 8 `f16` types, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m128h` type defined by Intel,"] # [doc = " representing a 128-bit SIMD register which internally is consisted of"] # [doc = " 8 packed `f16` instances. its purpose is for f16 related intrinsic"] # [doc = " implementations."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] pub struct __m128h (8 x f16) ; # [doc = " 256-bit wide set of 16 `f16` types, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m256h` type defined by Intel,"] # [doc = " representing a 256-bit SIMD register which internally is consisted of"] # [doc = " 16 packed `f16` instances. its purpose is for f16 related intrinsic"] # [doc = " implementations."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] pub struct __m256h (16 x f16) ; # [doc = " 512-bit wide set of 32 `f16` types, x86-specific"] # [doc = ""] # [doc = " This type is the same as the `__m512h` type defined by Intel,"] # [doc = " representing a 512-bit SIMD register which internally is consisted of"] # [doc = " 32 packed `f16` instances. its purpose is for f16 related intrinsic"] # [doc = " implementations."] # [doc = ""] # [doc = " The in-memory representation of this type is the same as the one of an"] # [doc = " equivalent array (i.e. the in-memory order of elements is the same, and"] # [doc = " there is no padding); however, the alignment is different and equal to"] # [doc = " the size of the type. Note that the ABI for function calls may *not* be"] # [doc = " the same."] pub struct __m512h (32 x f16) ; }}
mkitem!{mkstruct!{# [doc = " The BFloat16 type used in AVX-512 intrinsics."] # [repr (transparent)] # [derive (Copy , Clone , Debug)] # [allow (non_camel_case_types)] # [unstable (feature = "stdarch_x86_avx512_bf16" , issue = "127356")] pub struct bf16 (u16) ;}}
mkitem!{mkimpl!{impl bf16 { # [doc = " Raw transmutation from `u16`"] # [inline] # [must_use] # [unstable (feature = "stdarch_x86_avx512_bf16" , issue = "127356")] pub const fn from_bits (bits : u16) -> bf16 { bf16 (bits) } # [doc = " Raw transmutation to `u16`"] # [inline] # [must_use = "this returns the result of the operation, without modifying the original"] # [unstable (feature = "stdarch_x86_avx512_bf16" , issue = "127356")] pub const fn to_bits (self) -> u16 { self . 0 } }}}
mkitem!{# [doc = " The `__mmask64` type used in AVX-512 intrinsics, a 64-bit integer"] # [allow (non_camel_case_types)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub type __mmask64 = u64 ;}
mkitem!{# [doc = " The `__mmask32` type used in AVX-512 intrinsics, a 32-bit integer"] # [allow (non_camel_case_types)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub type __mmask32 = u32 ;}
mkitem!{# [doc = " The `__mmask16` type used in AVX-512 intrinsics, a 16-bit integer"] # [allow (non_camel_case_types)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub type __mmask16 = u16 ;}
mkitem!{# [doc = " The `__mmask8` type used in AVX-512 intrinsics, a 8-bit integer"] # [allow (non_camel_case_types)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub type __mmask8 = u8 ;}
mkitem!{# [doc = " The `_MM_CMPINT_ENUM` type used to specify comparison operations in AVX-512 intrinsics."] # [allow (non_camel_case_types)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub type _MM_CMPINT_ENUM = i32 ;}
mkitem!{# [doc = " The `MM_MANTISSA_NORM_ENUM` type used to specify mantissa normalized operations in AVX-512 intrinsics."] # [allow (non_camel_case_types)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub type _MM_MANTISSA_NORM_ENUM = i32 ;}
mkitem!{# [doc = " The `MM_MANTISSA_SIGN_ENUM` type used to specify mantissa signed operations in AVX-512 intrinsics."] # [allow (non_camel_case_types)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub type _MM_MANTISSA_SIGN_ENUM = i32 ;}
mkitem!{# [doc = " The `MM_PERM_ENUM` type used to specify shuffle operations in AVX-512 intrinsics."] # [allow (non_camel_case_types)] # [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub type _MM_PERM_ENUM = i32 ;}
mkmod!{test, { 
                getname!(test);
                getsrc!(test);
                getpath!(test);
                get_deps!(test);
                get_crates!(test);
                mkinclude!(test);
                 
            }}
mkuse!{# [cfg (test)] pub use self :: test :: * ;}
mkitem!{macro_rules ! as_transmute { ($ from : ty => $ as_from : ident , $ ($ as_to : ident -> $ to : ident) ,* $ (,) ?) => { impl $ from { $ (# [inline] pub (crate) fn $ as_to (self) -> crate :: core_arch :: simd ::$ to { unsafe { transmute (self) } }) * } $ (impl crate :: core_arch :: simd ::$ to { # [inline] pub (crate) fn $ as_from (self) -> $ from { unsafe { transmute (self) } } }) * } ; }}
mkitem!{as_transmute ! (__m128i => as_m128i , as_u8x16 -> u8x16 , as_u16x8 -> u16x8 , as_u32x4 -> u32x4 , as_u64x2 -> u64x2 , as_i8x16 -> i8x16 , as_i16x8 -> i16x8 , as_i32x4 -> i32x4 , as_i64x2 -> i64x2 ,) ;}
mkitem!{as_transmute ! (__m256i => as_m256i , as_u8x32 -> u8x32 , as_u16x16 -> u16x16 , as_u32x8 -> u32x8 , as_u64x4 -> u64x4 , as_i8x32 -> i8x32 , as_i16x16 -> i16x16 , as_i32x8 -> i32x8 , as_i64x4 -> i64x4 ,) ;}
mkitem!{as_transmute ! (__m512i => as_m512i , as_u8x64 -> u8x64 , as_u16x32 -> u16x32 , as_u32x16 -> u32x16 , as_u64x8 -> u64x8 , as_i8x64 -> i8x64 , as_i16x32 -> i16x32 , as_i32x16 -> i32x16 , as_i64x8 -> i64x8 ,) ;}
mkitem!{as_transmute ! (__m128 => as_m128 , as_f32x4 -> f32x4) ;}
mkitem!{as_transmute ! (__m128d => as_m128d , as_f64x2 -> f64x2) ;}
mkitem!{as_transmute ! (__m256 => as_m256 , as_f32x8 -> f32x8) ;}
mkitem!{as_transmute ! (__m256d => as_m256d , as_f64x4 -> f64x4) ;}
mkitem!{as_transmute ! (__m512 => as_m512 , as_f32x16 -> f32x16) ;}
mkitem!{as_transmute ! (__m512d => as_m512d , as_f64x8 -> f64x8) ;}
mkitem!{as_transmute ! (__m128bh => as_m128bh , as_u16x8 -> u16x8 , as_u32x4 -> u32x4 , as_i16x8 -> i16x8 , as_i32x4 -> i32x4 ,) ;}
mkitem!{as_transmute ! (__m256bh => as_m256bh , as_u16x16 -> u16x16 , as_u32x8 -> u32x8 , as_i16x16 -> i16x16 , as_i32x8 -> i32x8 ,) ;}
mkitem!{as_transmute ! (__m512bh => as_m512bh , as_u16x32 -> u16x32 , as_u32x16 -> u32x16 , as_i16x32 -> i16x32 , as_i32x16 -> i32x16 ,) ;}
mkitem!{as_transmute ! (__m128h => as_m128h , as_f16x8 -> f16x8) ;}
mkitem!{as_transmute ! (__m256h => as_m256h , as_f16x16 -> f16x16) ;}
mkitem!{as_transmute ! (__m512h => as_m512h , as_f16x32 -> f16x32) ;}
mkmod!{eflags, { 
                getname!(eflags);
                getsrc!(eflags);
                getpath!(eflags);
                get_deps!(eflags);
                get_crates!(eflags);
                mkinclude!(eflags);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: eflags :: * ;}
mkmod!{fxsr, { 
                getname!(fxsr);
                getsrc!(fxsr);
                getpath!(fxsr);
                get_deps!(fxsr);
                get_crates!(fxsr);
                mkinclude!(fxsr);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: fxsr :: * ;}
mkmod!{bswap, { 
                getname!(bswap);
                getsrc!(bswap);
                getpath!(bswap);
                get_deps!(bswap);
                get_crates!(bswap);
                mkinclude!(bswap);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: bswap :: * ;}
mkmod!{rdtsc, { 
                getname!(rdtsc);
                getsrc!(rdtsc);
                getpath!(rdtsc);
                get_deps!(rdtsc);
                get_crates!(rdtsc);
                mkinclude!(rdtsc);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: rdtsc :: * ;}
mkmod!{cpuid, { 
                getname!(cpuid);
                getsrc!(cpuid);
                getpath!(cpuid);
                get_deps!(cpuid);
                get_crates!(cpuid);
                mkinclude!(cpuid);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: cpuid :: * ;}
mkmod!{xsave, { 
                getname!(xsave);
                getsrc!(xsave);
                getpath!(xsave);
                get_deps!(xsave);
                get_crates!(xsave);
                mkinclude!(xsave);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: xsave :: * ;}
mkmod!{sse, { 
                getname!(sse);
                getsrc!(sse);
                getpath!(sse);
                get_deps!(sse);
                get_crates!(sse);
                mkinclude!(sse);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: sse :: * ;}
mkmod!{sse2, { 
                getname!(sse2);
                getsrc!(sse2);
                getpath!(sse2);
                get_deps!(sse2);
                get_crates!(sse2);
                mkinclude!(sse2);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: sse2 :: * ;}
mkmod!{sse3, { 
                getname!(sse3);
                getsrc!(sse3);
                getpath!(sse3);
                get_deps!(sse3);
                get_crates!(sse3);
                mkinclude!(sse3);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: sse3 :: * ;}
mkmod!{ssse3, { 
                getname!(ssse3);
                getsrc!(ssse3);
                getpath!(ssse3);
                get_deps!(ssse3);
                get_crates!(ssse3);
                mkinclude!(ssse3);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: ssse3 :: * ;}
mkmod!{sse41, { 
                getname!(sse41);
                getsrc!(sse41);
                getpath!(sse41);
                get_deps!(sse41);
                get_crates!(sse41);
                mkinclude!(sse41);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: sse41 :: * ;}
mkmod!{sse42, { 
                getname!(sse42);
                getsrc!(sse42);
                getpath!(sse42);
                get_deps!(sse42);
                get_crates!(sse42);
                mkinclude!(sse42);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: sse42 :: * ;}
mkmod!{avx, { 
                getname!(avx);
                getsrc!(avx);
                getpath!(avx);
                get_deps!(avx);
                get_crates!(avx);
                mkinclude!(avx);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: avx :: * ;}
mkmod!{avx2, { 
                getname!(avx2);
                getsrc!(avx2);
                getpath!(avx2);
                get_deps!(avx2);
                get_crates!(avx2);
                mkinclude!(avx2);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: avx2 :: * ;}
mkmod!{fma, { 
                getname!(fma);
                getsrc!(fma);
                getpath!(fma);
                get_deps!(fma);
                get_crates!(fma);
                mkinclude!(fma);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: fma :: * ;}
mkmod!{abm, { 
                getname!(abm);
                getsrc!(abm);
                getpath!(abm);
                get_deps!(abm);
                get_crates!(abm);
                mkinclude!(abm);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: abm :: * ;}
mkmod!{bmi1, { 
                getname!(bmi1);
                getsrc!(bmi1);
                getpath!(bmi1);
                get_deps!(bmi1);
                get_crates!(bmi1);
                mkinclude!(bmi1);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: bmi1 :: * ;}
mkmod!{bmi2, { 
                getname!(bmi2);
                getsrc!(bmi2);
                getpath!(bmi2);
                get_deps!(bmi2);
                get_crates!(bmi2);
                mkinclude!(bmi2);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: bmi2 :: * ;}
mkmod!{sse4a, { 
                getname!(sse4a);
                getsrc!(sse4a);
                getpath!(sse4a);
                get_deps!(sse4a);
                get_crates!(sse4a);
                mkinclude!(sse4a);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: sse4a :: * ;}
mkmod!{tbm, { 
                getname!(tbm);
                getsrc!(tbm);
                getpath!(tbm);
                get_deps!(tbm);
                get_crates!(tbm);
                mkinclude!(tbm);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: tbm :: * ;}
mkmod!{pclmulqdq, { 
                getname!(pclmulqdq);
                getsrc!(pclmulqdq);
                getpath!(pclmulqdq);
                get_deps!(pclmulqdq);
                get_crates!(pclmulqdq);
                mkinclude!(pclmulqdq);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: pclmulqdq :: * ;}
mkmod!{aes, { 
                getname!(aes);
                getsrc!(aes);
                getpath!(aes);
                get_deps!(aes);
                get_crates!(aes);
                mkinclude!(aes);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: aes :: * ;}
mkmod!{rdrand, { 
                getname!(rdrand);
                getsrc!(rdrand);
                getpath!(rdrand);
                get_deps!(rdrand);
                get_crates!(rdrand);
                mkinclude!(rdrand);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: rdrand :: * ;}
mkmod!{sha, { 
                getname!(sha);
                getsrc!(sha);
                getpath!(sha);
                get_deps!(sha);
                get_crates!(sha);
                mkinclude!(sha);
                 
            }}
mkuse!{# [stable (feature = "simd_x86" , since = "1.27.0")] pub use self :: sha :: * ;}
mkmod!{adx, { 
                getname!(adx);
                getsrc!(adx);
                getpath!(adx);
                get_deps!(adx);
                get_crates!(adx);
                mkinclude!(adx);
                 
            }}
mkuse!{# [stable (feature = "simd_x86_adx" , since = "1.33.0")] pub use self :: adx :: * ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkmod!{avx512f, { 
                getname!(avx512f);
                getsrc!(avx512f);
                getpath!(avx512f);
                get_deps!(avx512f);
                get_crates!(avx512f);
                mkinclude!(avx512f);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512f :: * ;}
mkmod!{avx512bw, { 
                getname!(avx512bw);
                getsrc!(avx512bw);
                getpath!(avx512bw);
                get_deps!(avx512bw);
                get_crates!(avx512bw);
                mkinclude!(avx512bw);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512bw :: * ;}
mkmod!{avx512cd, { 
                getname!(avx512cd);
                getsrc!(avx512cd);
                getpath!(avx512cd);
                get_deps!(avx512cd);
                get_crates!(avx512cd);
                mkinclude!(avx512cd);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512cd :: * ;}
mkmod!{avx512dq, { 
                getname!(avx512dq);
                getsrc!(avx512dq);
                getpath!(avx512dq);
                get_deps!(avx512dq);
                get_crates!(avx512dq);
                mkinclude!(avx512dq);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512dq :: * ;}
mkmod!{avx512ifma, { 
                getname!(avx512ifma);
                getsrc!(avx512ifma);
                getpath!(avx512ifma);
                get_deps!(avx512ifma);
                get_crates!(avx512ifma);
                mkinclude!(avx512ifma);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512ifma :: * ;}
mkmod!{avx512vbmi, { 
                getname!(avx512vbmi);
                getsrc!(avx512vbmi);
                getpath!(avx512vbmi);
                get_deps!(avx512vbmi);
                get_crates!(avx512vbmi);
                mkinclude!(avx512vbmi);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512vbmi :: * ;}
mkmod!{avx512vbmi2, { 
                getname!(avx512vbmi2);
                getsrc!(avx512vbmi2);
                getpath!(avx512vbmi2);
                get_deps!(avx512vbmi2);
                get_crates!(avx512vbmi2);
                mkinclude!(avx512vbmi2);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512vbmi2 :: * ;}
mkmod!{avx512vnni, { 
                getname!(avx512vnni);
                getsrc!(avx512vnni);
                getpath!(avx512vnni);
                get_deps!(avx512vnni);
                get_crates!(avx512vnni);
                mkinclude!(avx512vnni);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512vnni :: * ;}
mkmod!{avx512bitalg, { 
                getname!(avx512bitalg);
                getsrc!(avx512bitalg);
                getpath!(avx512bitalg);
                get_deps!(avx512bitalg);
                get_crates!(avx512bitalg);
                mkinclude!(avx512bitalg);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512bitalg :: * ;}
mkmod!{gfni, { 
                getname!(gfni);
                getsrc!(gfni);
                getpath!(gfni);
                get_deps!(gfni);
                get_crates!(gfni);
                mkinclude!(gfni);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: gfni :: * ;}
mkmod!{avx512vpopcntdq, { 
                getname!(avx512vpopcntdq);
                getsrc!(avx512vpopcntdq);
                getpath!(avx512vpopcntdq);
                get_deps!(avx512vpopcntdq);
                get_crates!(avx512vpopcntdq);
                mkinclude!(avx512vpopcntdq);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512vpopcntdq :: * ;}
mkmod!{vaes, { 
                getname!(vaes);
                getsrc!(vaes);
                getpath!(vaes);
                get_deps!(vaes);
                get_crates!(vaes);
                mkinclude!(vaes);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: vaes :: * ;}
mkmod!{vpclmulqdq, { 
                getname!(vpclmulqdq);
                getsrc!(vpclmulqdq);
                getpath!(vpclmulqdq);
                get_deps!(vpclmulqdq);
                get_crates!(vpclmulqdq);
                mkinclude!(vpclmulqdq);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: vpclmulqdq :: * ;}
mkmod!{bt, { 
                getname!(bt);
                getsrc!(bt);
                getpath!(bt);
                get_deps!(bt);
                get_crates!(bt);
                mkinclude!(bt);
                 
            }}
mkuse!{# [stable (feature = "simd_x86_bittest" , since = "1.55.0")] pub use self :: bt :: * ;}
mkmod!{rtm, { 
                getname!(rtm);
                getsrc!(rtm);
                getpath!(rtm);
                get_deps!(rtm);
                get_crates!(rtm);
                mkinclude!(rtm);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_x86_rtm" , issue = "111138")] pub use self :: rtm :: * ;}
mkmod!{f16c, { 
                getname!(f16c);
                getsrc!(f16c);
                getpath!(f16c);
                get_deps!(f16c);
                get_crates!(f16c);
                mkinclude!(f16c);
                 
            }}
mkuse!{# [stable (feature = "x86_f16c_intrinsics" , since = "1.68.0")] pub use self :: f16c :: * ;}
mkmod!{avx512bf16, { 
                getname!(avx512bf16);
                getsrc!(avx512bf16);
                getpath!(avx512bf16);
                get_deps!(avx512bf16);
                get_crates!(avx512bf16);
                mkinclude!(avx512bf16);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avx512bf16 :: * ;}
mkmod!{avxneconvert, { 
                getname!(avxneconvert);
                getsrc!(avxneconvert);
                getpath!(avxneconvert);
                get_deps!(avxneconvert);
                get_crates!(avxneconvert);
                mkinclude!(avxneconvert);
                 
            }}
mkuse!{# [stable (feature = "stdarch_x86_avx512" , since = "1.89")] pub use self :: avxneconvert :: * ;}
mkmod!{avx512fp16, { 
                getname!(avx512fp16);
                getsrc!(avx512fp16);
                getpath!(avx512fp16);
                get_deps!(avx512fp16);
                get_crates!(avx512fp16);
                mkinclude!(avx512fp16);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_x86_avx512_f16" , issue = "127213")] pub use self :: avx512fp16 :: * ;}
mkmod!{kl, { 
                getname!(kl);
                getsrc!(kl);
                getpath!(kl);
                get_deps!(kl);
                get_crates!(kl);
                mkinclude!(kl);
                 
            }}
mkuse!{# [stable (feature = "keylocker_x86" , since = "1.89.0")] pub use self :: kl :: * ;}