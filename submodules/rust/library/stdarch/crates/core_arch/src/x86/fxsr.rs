mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.fxsave"] fn fxsave (p : * mut u8) ; # [link_name = "llvm.x86.fxrstor"] fn fxrstor (p : * const u8) ; }}

macro_rules! _fxsave_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _fxsave in module {}", module_path!());
    };
}

mkfn!{
    _fxsave_introspect!();
    # [doc = " Saves the `x87` FPU, `MMX` technology, `XMM`, and `MXCSR` registers to the"] # [doc = " 512-byte-long 16-byte-aligned memory region `mem_addr`."] # [doc = ""] # [doc = " A misaligned destination operand raises a general-protection (#GP) or an"] # [doc = " alignment check exception (#AC)."] # [doc = ""] # [doc = " See [`FXSAVE`][fxsave] and [`FXRSTOR`][fxrstor]."] # [doc = ""] # [doc = " [fxsave]: http://www.felixcloutier.com/x86/FXSAVE.html"] # [doc = " [fxrstor]: http://www.felixcloutier.com/x86/FXRSTOR.html"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_fxsave)"] # [inline] # [target_feature (enable = "fxsr")] # [cfg_attr (test , assert_instr (fxsave))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _fxsave (mem_addr : * mut u8) { fxsave (mem_addr) }
}

macro_rules! _fxrstor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _fxrstor in module {}", module_path!());
    };
}

mkfn!{
    _fxrstor_introspect!();
    # [doc = " Restores the `XMM`, `MMX`, `MXCSR`, and `x87` FPU registers from the"] # [doc = " 512-byte-long 16-byte-aligned memory region `mem_addr`."] # [doc = ""] # [doc = " The contents of this memory region should have been written to by a"] # [doc = " previous"] # [doc = " `_fxsave` or `_fxsave64` intrinsic."] # [doc = ""] # [doc = " A misaligned destination operand raises a general-protection (#GP) or an"] # [doc = " alignment check exception (#AC)."] # [doc = ""] # [doc = " See [`FXSAVE`][fxsave] and [`FXRSTOR`][fxrstor]."] # [doc = ""] # [doc = " [fxsave]: http://www.felixcloutier.com/x86/FXSAVE.html"] # [doc = " [fxrstor]: http://www.felixcloutier.com/x86/FXRSTOR.html"] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_fxrstor)"] # [inline] # [target_feature (enable = "fxsr")] # [cfg_attr (test , assert_instr (fxrstor))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn _fxrstor (mem_addr : * const u8) { fxrstor (mem_addr) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: x86 :: * ;}
mkuse!{use std :: { cmp :: PartialEq , fmt } ;}
mkuse!{use stdarch_test :: simd_test ;}
mkitem!{mkstruct!{# [repr (align (16))] struct FxsaveArea { data : [u8 ; 512] , }}}
mkitem!{mkimpl!{impl FxsaveArea { fn new () -> FxsaveArea { FxsaveArea { data : [0 ; 512] } } fn ptr (& mut self) -> * mut u8 { self . data . as_mut_ptr () } }}}

macro_rules! test_fxsave_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_fxsave in module {}", module_path!());
    };
}

mkfn!{
    test_fxsave_introspect!();
    # [simd_test (enable = "fxsr")] # [cfg_attr (miri , ignore)] unsafe fn test_fxsave () { let mut a = FxsaveArea :: new () ; let mut b = FxsaveArea :: new () ; fxsr :: _fxsave (a . ptr ()) ; fxsr :: _fxrstor (a . ptr ()) ; fxsr :: _fxsave (b . ptr ()) ; }
} 
            }}