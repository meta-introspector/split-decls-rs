mkuse!{use crate :: arch :: asm ;}

macro_rules! __readeflags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __readeflags in module {}", module_path!());
    };
}

mkfn!{
    __readeflags_introspect!();
    # [doc = " Reads EFLAGS."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__readeflags)"] # [cfg (target_arch = "x86")] # [inline (always)] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.29.0" , note = "See issue #51810 - use inline assembly instead")] # [doc (hidden)] pub unsafe fn __readeflags () -> u32 { let eflags : u32 ; asm ! ("pushfd" , "pop {}" , out (reg) eflags , options (nomem , att_syntax)) ; eflags }
}

macro_rules! __readeflags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __readeflags in module {}", module_path!());
    };
}

mkfn!{
    __readeflags_introspect!();
    # [doc = " Reads EFLAGS."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__readeflags)"] # [cfg (target_arch = "x86_64")] # [inline (always)] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.29.0" , note = "See issue #51810 - use inline assembly instead")] # [doc (hidden)] pub unsafe fn __readeflags () -> u64 { let eflags : u64 ; asm ! ("pushfq" , "pop {}" , out (reg) eflags , options (nomem , att_syntax)) ; eflags }
}

macro_rules! __writeeflags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __writeeflags in module {}", module_path!());
    };
}

mkfn!{
    __writeeflags_introspect!();
    # [doc = " Write EFLAGS."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__writeeflags)"] # [cfg (target_arch = "x86")] # [inline (always)] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.29.0" , note = "See issue #51810 - use inline assembly instead")] # [doc (hidden)] pub unsafe fn __writeeflags (eflags : u32) { asm ! ("push {}" , "popfd" , in (reg) eflags , options (nomem , att_syntax)) ; }
}

macro_rules! __writeeflags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __writeeflags in module {}", module_path!());
    };
}

mkfn!{
    __writeeflags_introspect!();
    # [doc = " Write EFLAGS."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__writeeflags)"] # [cfg (target_arch = "x86_64")] # [inline (always)] # [stable (feature = "simd_x86" , since = "1.27.0")] # [deprecated (since = "1.29.0" , note = "See issue #51810 - use inline assembly instead")] # [doc (hidden)] pub unsafe fn __writeeflags (eflags : u64) { asm ! ("push {}" , "popfq" , in (reg) eflags , options (nomem , att_syntax)) ; }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: x86 :: * ;}

macro_rules! test_readeflags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_readeflags in module {}", module_path!());
    };
}

mkfn!{
    test_readeflags_introspect!();
    # [test] # [cfg_attr (miri , ignore)] # [allow (deprecated)] fn test_readeflags () { unsafe { let v = __readeflags () ; __writeeflags (v) ; let u = __readeflags () ; assert_eq ! (v , u) ; } }
} 
            }}