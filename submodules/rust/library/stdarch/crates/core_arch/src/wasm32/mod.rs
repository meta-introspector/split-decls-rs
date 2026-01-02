mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkmod!{atomic, { 
                getname!(atomic);
                getsrc!(atomic);
                getpath!(atomic);
                get_deps!(atomic);
                get_crates!(atomic);
                mkinclude!(atomic);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_wasm_atomic_wait" , issue = "77839")] pub use self :: atomic :: * ;}
mkmod!{simd128, { 
                getname!(simd128);
                getsrc!(simd128);
                getpath!(simd128);
                get_deps!(simd128);
                get_crates!(simd128);
                mkinclude!(simd128);
                 
            }}
mkuse!{# [stable (feature = "wasm_simd" , since = "1.54.0")] pub use self :: simd128 :: * ;}
mkmod!{relaxed_simd, { 
                getname!(relaxed_simd);
                getsrc!(relaxed_simd);
                getpath!(relaxed_simd);
                get_deps!(relaxed_simd);
                get_crates!(relaxed_simd);
                mkinclude!(relaxed_simd);
                 
            }}
mkuse!{# [stable (feature = "stdarch_wasm_relaxed_simd" , since = "1.82.0")] pub use self :: relaxed_simd :: * ;}
mkmod!{memory, { 
                getname!(memory);
                getsrc!(memory);
                getpath!(memory);
                get_deps!(memory);
                get_crates!(memory);
                mkinclude!(memory);
                 
            }}
mkuse!{# [stable (feature = "simd_wasm32" , since = "1.33.0")] pub use self :: memory :: * ;}

macro_rules! unreachable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unreachable in module {}", module_path!());
    };
}

mkfn!{
    unreachable_introspect!();
    # [doc = " Generates the [`unreachable`] instruction, which causes an unconditional [trap]."] # [doc = ""] # [doc = " This function is safe to call and immediately aborts the execution."] # [doc = ""] # [doc = " [`unreachable`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-control"] # [doc = " [trap]: https://webassembly.github.io/spec/core/intro/overview.html#trap"] # [cfg_attr (test , assert_instr (unreachable))] # [inline] # [stable (feature = "unreachable_wasm32" , since = "1.37.0")] pub fn unreachable () -> ! { crate :: intrinsics :: abort () }
}

macro_rules! f32_ceil_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f32_ceil in module {}", module_path!());
    };
}

mkfn!{
    f32_ceil_introspect!();
    # [doc = " Generates the [`f32.ceil`] instruction, returning the smallest integer greater than or equal to `a`."] # [doc = ""] # [doc = " This method is useful when targeting `no_std` and is equivalent to [`std::f32::ceil()`]."] # [doc = ""] # [doc = " [`std::f32::ceil()`]: https://doc.rust-lang.org/std/primitive.f32.html#method.ceil"] # [doc = " [`f32.ceil`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric"] # [cfg_attr (test , assert_instr (f32 . ceil))] # [inline] # [must_use = "method returns a new number and does not mutate the original value"] # [unstable (feature = "wasm_numeric_instr" , issue = "133908")] pub fn f32_ceil (a : f32) -> f32 { unsafe { crate :: intrinsics :: ceilf32 (a) } }
}

macro_rules! f32_floor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f32_floor in module {}", module_path!());
    };
}

mkfn!{
    f32_floor_introspect!();
    # [doc = " Generates the [`f32.floor`] instruction, returning the largest integer less than or equal to `a`."] # [doc = ""] # [doc = " This method is useful when targeting `no_std` and is equivalent to [`std::f32::floor()`]."] # [doc = ""] # [doc = " [`std::f32::floor()`]: https://doc.rust-lang.org/std/primitive.f32.html#method.floor"] # [doc = " [`f32.floor`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric"] # [cfg_attr (test , assert_instr (f32 . floor))] # [inline] # [must_use = "method returns a new number and does not mutate the original value"] # [unstable (feature = "wasm_numeric_instr" , issue = "133908")] pub fn f32_floor (a : f32) -> f32 { unsafe { crate :: intrinsics :: floorf32 (a) } }
}

macro_rules! f32_trunc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f32_trunc in module {}", module_path!());
    };
}

mkfn!{
    f32_trunc_introspect!();
    # [doc = " Generates the [`f32.trunc`] instruction, roundinging to the nearest integer towards zero."] # [doc = ""] # [doc = " This method is useful when targeting `no_std` and is equivalent to [`std::f32::trunc()`]."] # [doc = ""] # [doc = " [`std::f32::trunc()`]: https://doc.rust-lang.org/std/primitive.f32.html#method.trunc"] # [doc = " [`f32.trunc`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric"] # [cfg_attr (test , assert_instr (f32 . trunc))] # [inline] # [must_use = "method returns a new number and does not mutate the original value"] # [unstable (feature = "wasm_numeric_instr" , issue = "133908")] pub fn f32_trunc (a : f32) -> f32 { unsafe { crate :: intrinsics :: truncf32 (a) } }
}

macro_rules! f32_nearest_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f32_nearest in module {}", module_path!());
    };
}

mkfn!{
    f32_nearest_introspect!();
    # [doc = " Generates the [`f32.nearest`] instruction, roundinging to the nearest integer. Rounds half-way"] # [doc = " cases to the number with an even least significant digit."] # [doc = ""] # [doc = " This method is useful when targeting `no_std` and is equivalent to [`std::f32::round_ties_even()`]."] # [doc = ""] # [doc = " [`std::f32::round_ties_even()`]: https://doc.rust-lang.org/std/primitive.f32.html#method.round_ties_even"] # [doc = " [`f32.nearest`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric"] # [cfg_attr (test , assert_instr (f32 . nearest))] # [inline] # [must_use = "method returns a new number and does not mutate the original value"] # [unstable (feature = "wasm_numeric_instr" , issue = "133908")] pub fn f32_nearest (a : f32) -> f32 { crate :: intrinsics :: round_ties_even_f32 (a) }
}

macro_rules! f32_sqrt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f32_sqrt in module {}", module_path!());
    };
}

mkfn!{
    f32_sqrt_introspect!();
    # [doc = " Generates the [`f32.sqrt`] instruction, returning the square root of the number `a`."] # [doc = ""] # [doc = " This method is useful when targeting `no_std` and is equivalent to [`std::f32::sqrt()`]."] # [doc = ""] # [doc = " [`std::f32::sqrt()`]: https://doc.rust-lang.org/std/primitive.f32.html#method.sqrt"] # [doc = " [`f32.sqrt`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric"] # [cfg_attr (test , assert_instr (f32 . sqrt))] # [inline] # [must_use = "method returns a new number and does not mutate the original value"] # [unstable (feature = "wasm_numeric_instr" , issue = "133908")] pub fn f32_sqrt (a : f32) -> f32 { unsafe { crate :: intrinsics :: sqrtf32 (a) } }
}

macro_rules! f64_ceil_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f64_ceil in module {}", module_path!());
    };
}

mkfn!{
    f64_ceil_introspect!();
    # [doc = " Generates the [`f64.ceil`] instruction, returning the smallest integer greater than or equal to `a`."] # [doc = ""] # [doc = " This method is useful when targeting `no_std` and is equivalent to [`std::f64::ceil()`]."] # [doc = ""] # [doc = " [`std::f64::ceil()`]: https://doc.rust-lang.org/std/primitive.f64.html#method.ceil"] # [doc = " [`f64.ceil`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric"] # [cfg_attr (test , assert_instr (f64 . ceil))] # [inline] # [must_use = "method returns a new number and does not mutate the original value"] # [unstable (feature = "wasm_numeric_instr" , issue = "133908")] pub fn f64_ceil (a : f64) -> f64 { unsafe { crate :: intrinsics :: ceilf64 (a) } }
}

macro_rules! f64_floor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f64_floor in module {}", module_path!());
    };
}

mkfn!{
    f64_floor_introspect!();
    # [doc = " Generates the [`f64.floor`] instruction, returning the largest integer less than or equal to `a`."] # [doc = ""] # [doc = " This method is useful when targeting `no_std` and is equivalent to [`std::f64::floor()`]."] # [doc = ""] # [doc = " [`std::f64::floor()`]: https://doc.rust-lang.org/std/primitive.f64.html#method.floor"] # [doc = " [`f64.floor`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric"] # [cfg_attr (test , assert_instr (f64 . floor))] # [inline] # [must_use = "method returns a new number and does not mutate the original value"] # [unstable (feature = "wasm_numeric_instr" , issue = "133908")] pub fn f64_floor (a : f64) -> f64 { unsafe { crate :: intrinsics :: floorf64 (a) } }
}

macro_rules! f64_trunc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f64_trunc in module {}", module_path!());
    };
}

mkfn!{
    f64_trunc_introspect!();
    # [doc = " Generates the [`f64.trunc`] instruction, roundinging to the nearest integer towards zero."] # [doc = ""] # [doc = " This method is useful when targeting `no_std` and is equivalent to [`std::f64::trunc()`]."] # [doc = ""] # [doc = " [`std::f64::trunc()`]: https://doc.rust-lang.org/std/primitive.f64.html#method.trunc"] # [doc = " [`f64.trunc`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric"] # [cfg_attr (test , assert_instr (f64 . trunc))] # [inline] # [must_use = "method returns a new number and does not mutate the original value"] # [unstable (feature = "wasm_numeric_instr" , issue = "133908")] pub fn f64_trunc (a : f64) -> f64 { unsafe { crate :: intrinsics :: truncf64 (a) } }
}

macro_rules! f64_nearest_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f64_nearest in module {}", module_path!());
    };
}

mkfn!{
    f64_nearest_introspect!();
    # [doc = " Generates the [`f64.nearest`] instruction, roundinging to the nearest integer. Rounds half-way"] # [doc = " cases to the number with an even least significant digit."] # [doc = ""] # [doc = " This method is useful when targeting `no_std` and is equivalent to [`std::f64::round_ties_even()`]."] # [doc = ""] # [doc = " [`std::f64::round_ties_even()`]: https://doc.rust-lang.org/std/primitive.f64.html#method.round_ties_even"] # [doc = " [`f64.nearest`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric"] # [cfg_attr (test , assert_instr (f64 . nearest))] # [inline] # [must_use = "method returns a new number and does not mutate the original value"] # [unstable (feature = "wasm_numeric_instr" , issue = "133908")] pub fn f64_nearest (a : f64) -> f64 { crate :: intrinsics :: round_ties_even_f64 (a) }
}

macro_rules! f64_sqrt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f64_sqrt in module {}", module_path!());
    };
}

mkfn!{
    f64_sqrt_introspect!();
    # [doc = " Generates the [`f64.sqrt`] instruction, returning the square root of the number `a`."] # [doc = ""] # [doc = " This method is useful when targeting `no_std` and is equivalent to [`std::f64::sqrt()`]."] # [doc = ""] # [doc = " [`std::f64::sqrt()`]: https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt"] # [doc = " [`f64.sqrt`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric"] # [cfg_attr (test , assert_instr (f64 . sqrt))] # [inline] # [must_use = "method returns a new number and does not mutate the original value"] # [unstable (feature = "wasm_numeric_instr" , issue = "133908")] pub fn f64_sqrt (a : f64) -> f64 { unsafe { crate :: intrinsics :: sqrtf64 (a) } }
}
mkitem!{unsafe extern "C-unwind" { # [link_name = "llvm.wasm.throw"] fn wasm_throw (tag : i32 , ptr : * mut u8) -> ! ; }}

macro_rules! throw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function throw in module {}", module_path!());
    };
}

mkfn!{
    throw_introspect!();
    # [doc = " Generates the [`throw`] instruction from the [exception-handling proposal] for WASM."] # [doc = ""] # [doc = " This function is unlikely to be stabilized until codegen backends have better support."] # [doc = ""] # [doc = " [`throw`]: https://webassembly.github.io/exception-handling/core/syntax/instructions.html#syntax-instr-control"] # [doc = " [exception-handling proposal]: https://github.com/WebAssembly/exception-handling"] # [inline] # [unstable (feature = "wasm_exception_handling_intrinsics" , issue = "122465")] # [allow (ffi_unwind_calls)] pub unsafe fn throw < const TAG : i32 > (ptr : * mut u8) -> ! { static_assert ! (TAG == 0) ; wasm_throw (TAG , ptr) }
}