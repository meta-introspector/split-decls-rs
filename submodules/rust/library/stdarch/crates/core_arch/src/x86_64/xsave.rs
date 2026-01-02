mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.xsave64"] fn xsave64 (p : * mut u8 , hi : u32 , lo : u32) ; # [link_name = "llvm.x86.xrstor64"] fn xrstor64 (p : * const u8 , hi : u32 , lo : u32) ; # [link_name = "llvm.x86.xsaveopt64"] fn xsaveopt64 (p : * mut u8 , hi : u32 , lo : u32) ; # [link_name = "llvm.x86.xsavec64"] fn xsavec64 (p : * mut u8 , hi : u32 , lo : u32) ; # [link_name = "llvm.x86.xsaves64"] fn xsaves64 (p : * mut u8 , hi : u32 , lo : u32) ; # [link_name = "llvm.x86.xrstors64"] fn xrstors64 (p : * const u8 , hi : u32 , lo : u32) ; }}

macro_rules! _xsave64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xsave64 in module {}", module_path!());
    };
}

mkfn!{
    _xsave64_introspect!();
    # [doc = " Performs a full or partial save of the enabled processor states to memory at"] # [doc = " `mem_addr`."] # [doc = ""] # [doc = " State is saved based on bits `[62:0]` in `save_mask` and XCR0."] # [doc = " `mem_addr` must be aligned on a 64-byte boundary."] # [doc = ""] # [doc = " The format of the XSAVE area is detailed in Section 13.4, “XSAVE Area,” of"] # [doc = " Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xsave64)"] # [inline] # [target_feature (enable = "xsave")] # [cfg_attr (test , assert_instr (xsave64))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xsave64 (mem_addr : * mut u8 , save_mask : u64) { xsave64 (mem_addr , (save_mask >> 32) as u32 , save_mask as u32) ; }
}

macro_rules! _xrstor64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xrstor64 in module {}", module_path!());
    };
}

mkfn!{
    _xrstor64_introspect!();
    # [doc = " Performs a full or partial restore of the enabled processor states using"] # [doc = " the state information stored in memory at `mem_addr`."] # [doc = ""] # [doc = " State is restored based on bits `[62:0]` in `rs_mask`, `XCR0`, and"] # [doc = " `mem_addr.HEADER.XSTATE_BV`. `mem_addr` must be aligned on a 64-byte"] # [doc = " boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xrstor64)"] # [inline] # [target_feature (enable = "xsave")] # [cfg_attr (test , assert_instr (xrstor64))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xrstor64 (mem_addr : * const u8 , rs_mask : u64) { xrstor64 (mem_addr , (rs_mask >> 32) as u32 , rs_mask as u32) ; }
}

macro_rules! _xsaveopt64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xsaveopt64 in module {}", module_path!());
    };
}

mkfn!{
    _xsaveopt64_introspect!();
    # [doc = " Performs a full or partial save of the enabled processor states to memory at"] # [doc = " `mem_addr`."] # [doc = ""] # [doc = " State is saved based on bits `[62:0]` in `save_mask` and `XCR0`."] # [doc = " `mem_addr` must be aligned on a 64-byte boundary. The hardware may optimize"] # [doc = " the manner in which data is saved. The performance of this instruction will"] # [doc = " be equal to or better than using the `XSAVE64` instruction."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xsaveopt64)"] # [inline] # [target_feature (enable = "xsave,xsaveopt")] # [cfg_attr (test , assert_instr (xsaveopt64))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xsaveopt64 (mem_addr : * mut u8 , save_mask : u64) { xsaveopt64 (mem_addr , (save_mask >> 32) as u32 , save_mask as u32) ; }
}

macro_rules! _xsavec64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xsavec64 in module {}", module_path!());
    };
}

mkfn!{
    _xsavec64_introspect!();
    # [doc = " Performs a full or partial save of the enabled processor states to memory"] # [doc = " at `mem_addr`."] # [doc = ""] # [doc = " `xsavec` differs from `xsave` in that it uses compaction and that it may"] # [doc = " use init optimization. State is saved based on bits `[62:0]` in `save_mask`"] # [doc = " and `XCR0`. `mem_addr` must be aligned on a 64-byte boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xsavec64)"] # [inline] # [target_feature (enable = "xsave,xsavec")] # [cfg_attr (test , assert_instr (xsavec64))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xsavec64 (mem_addr : * mut u8 , save_mask : u64) { xsavec64 (mem_addr , (save_mask >> 32) as u32 , save_mask as u32) ; }
}

macro_rules! _xsaves64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xsaves64 in module {}", module_path!());
    };
}

mkfn!{
    _xsaves64_introspect!();
    # [doc = " Performs a full or partial save of the enabled processor states to memory at"] # [doc = " `mem_addr`"] # [doc = ""] # [doc = " `xsaves` differs from xsave in that it can save state components"] # [doc = " corresponding to bits set in `IA32_XSS` `MSR` and that it may use the"] # [doc = " modified optimization. State is saved based on bits `[62:0]` in `save_mask`"] # [doc = " and `XCR0`. `mem_addr` must be aligned on a 64-byte boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xsaves64)"] # [inline] # [target_feature (enable = "xsave,xsaves")] # [cfg_attr (test , assert_instr (xsaves64))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xsaves64 (mem_addr : * mut u8 , save_mask : u64) { xsaves64 (mem_addr , (save_mask >> 32) as u32 , save_mask as u32) ; }
}

macro_rules! _xrstors64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _xrstors64 in module {}", module_path!());
    };
}

mkfn!{
    _xrstors64_introspect!();
    # [doc = " Performs a full or partial restore of the enabled processor states using the"] # [doc = " state information stored in memory at `mem_addr`."] # [doc = ""] # [doc = " `xrstors` differs from `xrstor` in that it can restore state components"] # [doc = " corresponding to bits set in the `IA32_XSS` `MSR`; `xrstors` cannot restore"] # [doc = " from an `xsave` area in which the extended region is in the standard form."] # [doc = " State is restored based on bits `[62:0]` in `rs_mask`, `XCR0`, and"] # [doc = " `mem_addr.HEADER.XSTATE_BV`. `mem_addr` must be aligned on a 64-byte"] # [doc = " boundary."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_xrstors64)"] # [inline] # [target_feature (enable = "xsave,xsaves")] # [cfg_attr (test , assert_instr (xrstors64))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _xrstors64 (mem_addr : * const u8 , rs_mask : u64) { xrstors64 (mem_addr , (rs_mask >> 32) as u32 , rs_mask as u32) ; }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: x86_64 :: xsave ;}
mkuse!{use std :: fmt ;}
mkuse!{use stdarch_test :: simd_test ;}
mkitem!{mkstruct!{# [repr (align (64))] # [derive (Debug)] struct XsaveArea { data : [u8 ; 2560] , }}}
mkitem!{mkimpl!{impl XsaveArea { fn new () -> XsaveArea { XsaveArea { data : [0 ; 2560] } } fn ptr (& mut self) -> * mut u8 { self . data . as_mut_ptr () } }}}

macro_rules! test_xsave64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_xsave64 in module {}", module_path!());
    };
}

mkfn!{
    test_xsave64_introspect!();
    # [simd_test (enable = "xsave")] # [cfg_attr (miri , ignore)] unsafe fn test_xsave64 () { let m = 0xFFFFFFFFFFFFFFFF_u64 ; let mut a = XsaveArea :: new () ; let mut b = XsaveArea :: new () ; xsave :: _xsave64 (a . ptr () , m) ; xsave :: _xrstor64 (a . ptr () , m) ; xsave :: _xsave64 (b . ptr () , m) ; }
}

macro_rules! test_xsaveopt64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_xsaveopt64 in module {}", module_path!());
    };
}

mkfn!{
    test_xsaveopt64_introspect!();
    # [simd_test (enable = "xsave,xsaveopt")] # [cfg_attr (miri , ignore)] unsafe fn test_xsaveopt64 () { let m = 0xFFFFFFFFFFFFFFFF_u64 ; let mut a = XsaveArea :: new () ; let mut b = XsaveArea :: new () ; xsave :: _xsaveopt64 (a . ptr () , m) ; xsave :: _xrstor64 (a . ptr () , m) ; xsave :: _xsaveopt64 (b . ptr () , m) ; }
}

macro_rules! test_xsavec64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_xsavec64 in module {}", module_path!());
    };
}

mkfn!{
    test_xsavec64_introspect!();
    # [simd_test (enable = "xsave,xsavec")] # [cfg_attr (miri , ignore)] unsafe fn test_xsavec64 () { let m = 0xFFFFFFFFFFFFFFFF_u64 ; let mut a = XsaveArea :: new () ; let mut b = XsaveArea :: new () ; xsave :: _xsavec64 (a . ptr () , m) ; xsave :: _xrstor64 (a . ptr () , m) ; xsave :: _xsavec64 (b . ptr () , m) ; }
} 
            }}