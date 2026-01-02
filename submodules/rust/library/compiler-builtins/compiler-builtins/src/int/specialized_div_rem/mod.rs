mkmod!{norm_shift, { 
                getname!(norm_shift);
                getsrc!(norm_shift);
                getpath!(norm_shift);
                get_deps!(norm_shift);
                get_crates!(norm_shift);
                mkinclude!(norm_shift);
                 
            }}
mkmod!{binary_long, { 
                getname!(binary_long);
                getsrc!(binary_long);
                getpath!(binary_long);
                get_deps!(binary_long);
                get_crates!(binary_long);
                mkinclude!(binary_long);
                 
            }}
mkmod!{delegate, { 
                getname!(delegate);
                getsrc!(delegate);
                getpath!(delegate);
                get_deps!(delegate);
                get_crates!(delegate);
                mkinclude!(delegate);
                 
            }}
mkuse!{# [allow (unused_imports)] # [cfg (not (feature = "unstable-public-internals"))] pub (crate) use self :: delegate :: u128_divide_sparc ;}
mkuse!{# [cfg (feature = "unstable-public-internals")] pub use self :: delegate :: u128_divide_sparc ;}
mkmod!{trifecta, { 
                getname!(trifecta);
                getsrc!(trifecta);
                getpath!(trifecta);
                get_deps!(trifecta);
                get_crates!(trifecta);
                mkinclude!(trifecta);
                 
            }}
mkmod!{asymmetric, { 
                getname!(asymmetric);
                getsrc!(asymmetric);
                getpath!(asymmetric);
                get_deps!(asymmetric);
                get_crates!(asymmetric);
                mkinclude!(asymmetric);
                 
            }}

macro_rules! zero_div_fn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zero_div_fn in module {}", module_path!());
    };
}

mkfn!{
    zero_div_fn_introspect!();
    # [doc = " The behavior of all divisions by zero is controlled by this function. This function should be"] # [doc = " impossible to reach by Rust users, unless `compiler-builtins` public division functions or"] # [doc = " `core/std::unchecked_div/rem` are directly used without a zero check in front."] fn zero_div_fn () -> ! { unsafe { core :: intrinsics :: unreachable () } }
}
mkitem!{const USE_LZ : bool = { if cfg ! (target_arch = "arm") { if cfg ! (target_feature = "thumb-mode") { cfg ! (target_feature = "v6t2") } else { cfg ! (target_feature = "v5te") } } else if cfg ! (any (target_arch = "sparc" , target_arch = "sparc64")) { cfg ! (target_feature = "vis3") } else if cfg ! (any (target_arch = "riscv32" , target_arch = "riscv64")) { cfg ! (target_feature = "zbb") } else { true } } ;}
mkitem!{impl_normalization_shift ! (u32_normalization_shift , USE_LZ , 32 , u32 , i32 , allow (dead_code)) ;}
mkitem!{impl_normalization_shift ! (u64_normalization_shift , USE_LZ , 64 , u64 , i64 , allow (dead_code)) ;}

macro_rules! u64_by_u64_div_rem_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u64_by_u64_div_rem in module {}", module_path!());
    };
}

mkfn!{
    u64_by_u64_div_rem_introspect!();
    # [doc = " Divides `duo` by `div` and returns a tuple of the quotient and the remainder."] # [doc = " `checked_div` and `checked_rem` are used to avoid bringing in panic function"] # [doc = " dependencies."] # [inline] fn u64_by_u64_div_rem (duo : u64 , div : u64) -> (u64 , u64) { if let Some (quo) = duo . checked_div (div) && let Some (rem) = duo . checked_rem (div) { return (quo , rem) ; } zero_div_fn () }
}
mkitem!{# [cfg (all (any (target_family = "wasm" , not (any (target_pointer_width = "16" , target_pointer_width = "32")) ,) , not (all (not (feature = "no-asm") , target_arch = "x86_64")) , not (any (target_arch = "sparc" , target_arch = "sparc64"))))] impl_trifecta ! (u128_div_rem , zero_div_fn , u64_by_u64_div_rem , 32 , u32 , u64 , u128) ;}
mkitem!{# [cfg (all (not (any (target_family = "wasm" , not (any (target_pointer_width = "16" , target_pointer_width = "32")) ,)) , not (all (not (feature = "no-asm") , target_arch = "x86_64")) , not (any (target_arch = "sparc" , target_arch = "sparc64"))))] impl_delegate ! (u128_div_rem , zero_div_fn , u64_normalization_shift , u64_by_u64_div_rem , 32 , u32 , u64 , u128 , i128) ;}

macro_rules! u128_by_u64_div_rem_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u128_by_u64_div_rem in module {}", module_path!());
    };
}

mkfn!{
    u128_by_u64_div_rem_introspect!();
    # [doc = " Divides `duo` by `div` and returns a tuple of the quotient and the remainder."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If the quotient does not fit in a `u64`, a floating point exception occurs."] # [doc = " If `div == 0`, then a division by zero exception occurs."] # [cfg (all (not (feature = "no-asm") , target_arch = "x86_64"))] # [inline] unsafe fn u128_by_u64_div_rem (duo : u128 , div : u64) -> (u64 , u64) { let duo_lo = duo as u64 ; let duo_hi = (duo >> 64) as u64 ; let quo : u64 ; let rem : u64 ; unsafe { core :: arch :: asm ! ("div {0}" , in (reg) div , inlateout ("rax") duo_lo => quo , inlateout ("rdx") duo_hi => rem , options (att_syntax , pure , nomem , nostack)) ; } (quo , rem) }
}
mkitem!{# [cfg (all (not (feature = "no-asm") , target_arch = "x86_64"))] impl_asymmetric ! (u128_div_rem , zero_div_fn , u64_by_u64_div_rem , u128_by_u64_div_rem , 32 , u32 , u64 , u128) ;}

macro_rules! u32_by_u32_div_rem_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u32_by_u32_div_rem in module {}", module_path!());
    };
}

mkfn!{
    u32_by_u32_div_rem_introspect!();
    # [doc = " Divides `duo` by `div` and returns a tuple of the quotient and the remainder."] # [doc = " `checked_div` and `checked_rem` are used to avoid bringing in panic function"] # [doc = " dependencies."] # [inline] # [allow (dead_code)] fn u32_by_u32_div_rem (duo : u32 , div : u32) -> (u32 , u32) { if let Some (quo) = duo . checked_div (div) && let Some (rem) = duo . checked_rem (div) { return (quo , rem) ; } zero_div_fn () }
}
mkitem!{# [cfg (all (not (all (not (feature = "no-asm") , target_arch = "x86")) , not (target_pointer_width = "64")))] impl_delegate ! (u64_div_rem , zero_div_fn , u32_normalization_shift , u32_by_u32_div_rem , 16 , u16 , u32 , u64 , i64) ;}
mkitem!{# [cfg (all (not (all (not (feature = "no-asm") , target_arch = "x86")) , target_pointer_width = "64"))] impl_binary_long ! (u64_div_rem , zero_div_fn , u64_normalization_shift , 64 , u64 , i64) ;}

macro_rules! u64_by_u32_div_rem_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u64_by_u32_div_rem in module {}", module_path!());
    };
}

mkfn!{
    u64_by_u32_div_rem_introspect!();
    # [doc = " Divides `duo` by `div` and returns a tuple of the quotient and the remainder."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If the quotient does not fit in a `u32`, a floating point exception occurs."] # [doc = " If `div == 0`, then a division by zero exception occurs."] # [cfg (all (not (feature = "no-asm") , target_arch = "x86"))] # [inline] unsafe fn u64_by_u32_div_rem (duo : u64 , div : u32) -> (u32 , u32) { let duo_lo = duo as u32 ; let duo_hi = (duo >> 32) as u32 ; let quo : u32 ; let rem : u32 ; unsafe { core :: arch :: asm ! ("div {0}" , in (reg) div , inlateout ("rax") duo_lo => quo , inlateout ("rdx") duo_hi => rem , options (att_syntax , pure , nomem , nostack)) ; } (quo , rem) }
}
mkitem!{# [cfg (all (not (feature = "no-asm") , target_arch = "x86"))] impl_asymmetric ! (u64_div_rem , zero_div_fn , u32_by_u32_div_rem , u64_by_u32_div_rem , 16 , u16 , u32 , u64) ;}
mkitem!{impl_binary_long ! (u32_div_rem , zero_div_fn , u32_normalization_shift , 32 , u32 , i32 , allow (dead_code)) ;}