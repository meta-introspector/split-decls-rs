mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.xsave"] fn xsave (p : * mut u8 , hi : u32 , lo : u32) ; # [link_name = "llvm.x86.xrstor"] fn xrstor (p : * const u8 , hi : u32 , lo : u32) ; # [link_name = "llvm.x86.xsetbv"] fn xsetbv (v : u32 , hi : u32 , lo : u32) ; # [link_name = "llvm.x86.xgetbv"] fn xgetbv (v : u32) -> i64 ; # [link_name = "llvm.x86.xsaveopt"] fn xsaveopt (p : * mut u8 , hi : u32 , lo : u32) ; # [link_name = "llvm.x86.xsavec"] fn xsavec (p : * mut u8 , hi : u32 , lo : u32) ; # [link_name = "llvm.x86.xsaves"] fn xsaves (p : * mut u8 , hi : u32 , lo : u32) ; # [link_name = "llvm.x86.xrstors"] fn xrstors (p : * const u8 , hi : u32 , lo : u32) ; }}

macro_rules! _xsave_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xsave in module {}", module_path!());
    };
}

mkfn!{
    _xsave_introspect!();
    # [doc = " Performs a full or partial save of the enabled processor states to memory at"] # [doc = " `mem_addr`."] # [doc = ""] # [doc = " State is saved based on bits `[62:0]` in `save_mask` and XCR0."] # [doc = " `mem_addr` must be aligned on a 64-byte boundary."] # [doc = ""] # [doc = " The format of the XSAVE area is detailed in Section 13.4, “XSAVE Area,” of"] # [doc = " Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xsave)"] # [inline] # [target_feature (enable = "xsave")] # [cfg_attr (test , assert_instr (xsave))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xsave (mem_addr : * mut u8 , save_mask : u64) { xsave (mem_addr , (save_mask >> 32) as u32 , save_mask as u32) ; }
}

macro_rules! _xrstor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xrstor in module {}", module_path!());
    };
}

mkfn!{
    _xrstor_introspect!();
    # [doc = " Performs a full or partial restore of the enabled processor states using"] # [doc = " the state information stored in memory at `mem_addr`."] # [doc = ""] # [doc = " State is restored based on bits `[62:0]` in `rs_mask`, `XCR0`, and"] # [doc = " `mem_addr.HEADER.XSTATE_BV`. `mem_addr` must be aligned on a 64-byte"] # [doc = " boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xrstor)"] # [inline] # [target_feature (enable = "xsave")] # [cfg_attr (test , assert_instr (xrstor))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xrstor (mem_addr : * const u8 , rs_mask : u64) { xrstor (mem_addr , (rs_mask >> 32) as u32 , rs_mask as u32) ; }
}
mkitem!{# [doc = " `XFEATURE_ENABLED_MASK` for `XCR`"] # [doc = ""] # [doc = " This intrinsic maps to `XSETBV` instruction."] # [stable (feature = "simd_x86" , since = "1.27.0")] pub const _XCR_XFEATURE_ENABLED_MASK : u32 = 0 ;}

macro_rules! _xsetbv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xsetbv in module {}", module_path!());
    };
}

mkfn!{
    _xsetbv_introspect!();
    # [doc = " Copies 64-bits from `val` to the extended control register (`XCR`) specified"] # [doc = " by `a`."] # [doc = ""] # [doc = " Currently only `XFEATURE_ENABLED_MASK` `XCR` is supported."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xsetbv)"] # [inline] # [target_feature (enable = "xsave")] # [cfg_attr (test , assert_instr (xsetbv))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xsetbv (a : u32 , val : u64) { xsetbv (a , (val >> 32) as u32 , val as u32) ; }
}

macro_rules! _xgetbv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xgetbv in module {}", module_path!());
    };
}

mkfn!{
    _xgetbv_introspect!();
    # [doc = " Reads the contents of the extended control register `XCR`"] # [doc = " specified in `xcr_no`."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xgetbv)"] # [inline] # [target_feature (enable = "xsave")] # [cfg_attr (test , assert_instr (xgetbv))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xgetbv (xcr_no : u32) -> u64 { xgetbv (xcr_no) as u64 }
}

macro_rules! _xsaveopt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xsaveopt in module {}", module_path!());
    };
}

mkfn!{
    _xsaveopt_introspect!();
    # [doc = " Performs a full or partial save of the enabled processor states to memory at"] # [doc = " `mem_addr`."] # [doc = ""] # [doc = " State is saved based on bits `[62:0]` in `save_mask` and `XCR0`."] # [doc = " `mem_addr` must be aligned on a 64-byte boundary. The hardware may optimize"] # [doc = " the manner in which data is saved. The performance of this instruction will"] # [doc = " be equal to or better than using the `XSAVE` instruction."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xsaveopt)"] # [inline] # [target_feature (enable = "xsave,xsaveopt")] # [cfg_attr (test , assert_instr (xsaveopt))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xsaveopt (mem_addr : * mut u8 , save_mask : u64) { xsaveopt (mem_addr , (save_mask >> 32) as u32 , save_mask as u32) ; }
}

macro_rules! _xsavec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xsavec in module {}", module_path!());
    };
}

mkfn!{
    _xsavec_introspect!();
    # [doc = " Performs a full or partial save of the enabled processor states to memory"] # [doc = " at `mem_addr`."] # [doc = ""] # [doc = " `xsavec` differs from `xsave` in that it uses compaction and that it may"] # [doc = " use init optimization. State is saved based on bits `[62:0]` in `save_mask`"] # [doc = " and `XCR0`. `mem_addr` must be aligned on a 64-byte boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xsavec)"] # [inline] # [target_feature (enable = "xsave,xsavec")] # [cfg_attr (test , assert_instr (xsavec))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xsavec (mem_addr : * mut u8 , save_mask : u64) { xsavec (mem_addr , (save_mask >> 32) as u32 , save_mask as u32) ; }
}

macro_rules! _xsaves_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xsaves in module {}", module_path!());
    };
}

mkfn!{
    _xsaves_introspect!();
    # [doc = " Performs a full or partial save of the enabled processor states to memory at"] # [doc = " `mem_addr`"] # [doc = ""] # [doc = " `xsaves` differs from xsave in that it can save state components"] # [doc = " corresponding to bits set in `IA32_XSS` `MSR` and that it may use the"] # [doc = " modified optimization. State is saved based on bits `[62:0]` in `save_mask`"] # [doc = " and `XCR0`. `mem_addr` must be aligned on a 64-byte boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xsaves)"] # [inline] # [target_feature (enable = "xsave,xsaves")] # [cfg_attr (test , assert_instr (xsaves))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xsaves (mem_addr : * mut u8 , save_mask : u64) { xsaves (mem_addr , (save_mask >> 32) as u32 , save_mask as u32) ; }
}

macro_rules! _xrstors_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xrstors in module {}", module_path!());
    };
}

mkfn!{
    _xrstors_introspect!();
    # [doc = " Performs a full or partial restore of the enabled processor states using the"] # [doc = " state information stored in memory at `mem_addr`."] # [doc = ""] # [doc = " `xrstors` differs from `xrstor` in that it can restore state components"] # [doc = " corresponding to bits set in the `IA32_XSS` `MSR`; `xrstors` cannot restore"] # [doc = " from an `xsave` area in which the extended region is in the standard form."] # [doc = " State is restored based on bits `[62:0]` in `rs_mask`, `XCR0`, and"] # [doc = " `mem_addr.HEADER.XSTATE_BV`. `mem_addr` must be aligned on a 64-byte"] # [doc = " boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xrstors)"] # [inline] # [target_feature (enable = "xsave,xsaves")] # [cfg_attr (test , assert_instr (xrstors))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xrstors (mem_addr : * const u8 , rs_mask : u64) { xrstors (mem_addr , (rs_mask >> 32) as u32 , rs_mask as u32) ; }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use std :: { fmt , prelude :: v1 :: * } ;}
mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{use stdarch_test :: simd_test ;}
mkitem!{mkstruct!{# [repr (align (64))] # [derive (Debug)] struct XsaveArea { data : [u8 ; 2560] , }}}
mkitem!{mkimpl!{impl XsaveArea { fn new () -> XsaveArea { XsaveArea { data : [0 ; 2560] } } fn ptr (& mut self) -> * mut u8 { self . data . as_mut_ptr () } }}}

macro_rules! test_xsave_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_xsave in module {}", module_path!());
    };
}

mkfn!{
    test_xsave_introspect!();
    # [simd_test (enable = "xsave")] # [cfg_attr (miri , ignore)] unsafe fn test_xsave () { let m = 0xFFFFFFFFFFFFFFFF_u64 ; let mut a = XsaveArea :: new () ; let mut b = XsaveArea :: new () ; _xsave (a . ptr () , m) ; _xrstor (a . ptr () , m) ; _xsave (b . ptr () , m) ; }
}

macro_rules! test_xgetbv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_xgetbv in module {}", module_path!());
    };
}

mkfn!{
    test_xgetbv_introspect!();
    # [simd_test (enable = "xsave")] # [cfg_attr (miri , ignore)] unsafe fn test_xgetbv () { let xcr_n : u32 = _XCR_XFEATURE_ENABLED_MASK ; let xcr : u64 = _xgetbv (xcr_n) ; let xcr_cpy : u64 = _xgetbv (xcr_n) ; assert_eq ! (xcr , xcr_cpy) ; }
}

macro_rules! test_xsaveopt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_xsaveopt in module {}", module_path!());
    };
}

mkfn!{
    test_xsaveopt_introspect!();
    # [simd_test (enable = "xsave,xsaveopt")] # [cfg_attr (miri , ignore)] unsafe fn test_xsaveopt () { let m = 0xFFFFFFFFFFFFFFFF_u64 ; let mut a = XsaveArea :: new () ; let mut b = XsaveArea :: new () ; _xsaveopt (a . ptr () , m) ; _xrstor (a . ptr () , m) ; _xsaveopt (b . ptr () , m) ; }
}

macro_rules! test_xsavec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_xsavec in module {}", module_path!());
    };
}

mkfn!{
    test_xsavec_introspect!();
    # [simd_test (enable = "xsave,xsavec")] # [cfg_attr (miri , ignore)] unsafe fn test_xsavec () { let m = 0xFFFFFFFFFFFFFFFF_u64 ; let mut a = XsaveArea :: new () ; let mut b = XsaveArea :: new () ; _xsavec (a . ptr () , m) ; _xrstor (a . ptr () , m) ; _xsavec (b . ptr () , m) ; }
} 
            }}