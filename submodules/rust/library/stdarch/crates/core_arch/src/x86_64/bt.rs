mkuse!{use crate :: arch :: asm ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [cfg (target_pointer_width = "32")] macro_rules ! bt { ($ inst : expr) => { concat ! ($ inst , " {b}, ({p:e})") } ; }}
mkitem!{# [cfg (target_pointer_width = "64")] macro_rules ! bt { ($ inst : expr) => { concat ! ($ inst , " {b}, ({p})") } ; }}

macro_rules! _bittest64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bittest64 in module {}", module_path!());
    };
}

mkfn!{
    _bittest64_introspect!();
    # [doc = " Returns the bit in position `b` of the memory addressed by `p`."] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_bittest64)"] # [inline] # [cfg_attr (test , assert_instr (bt))] # [stable (feature = "simd_x86_bittest" , since = "1.55.0")] pub unsafe fn _bittest64 (p : * const i64 , b : i64) -> u8 { let r : u8 ; asm ! (bt ! ("btq") , "setc {r}" , p = in (reg) p , b = in (reg) b , r = out (reg_byte) r , options (readonly , nostack , pure , att_syntax)) ; r }
}

macro_rules! _bittestandset64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bittestandset64 in module {}", module_path!());
    };
}

mkfn!{
    _bittestandset64_introspect!();
    # [doc = " Returns the bit in position `b` of the memory addressed by `p`, then sets the bit to `1`."] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_bittestandset64)"] # [inline] # [cfg_attr (test , assert_instr (bts))] # [stable (feature = "simd_x86_bittest" , since = "1.55.0")] pub unsafe fn _bittestandset64 (p : * mut i64 , b : i64) -> u8 { let r : u8 ; asm ! (bt ! ("btsq") , "setc {r}" , p = in (reg) p , b = in (reg) b , r = out (reg_byte) r , options (nostack , att_syntax)) ; r }
}

macro_rules! _bittestandreset64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bittestandreset64 in module {}", module_path!());
    };
}

mkfn!{
    _bittestandreset64_introspect!();
    # [doc = " Returns the bit in position `b` of the memory addressed by `p`, then resets that bit to `0`."] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_bittestandreset64)"] # [inline] # [cfg_attr (test , assert_instr (btr))] # [stable (feature = "simd_x86_bittest" , since = "1.55.0")] pub unsafe fn _bittestandreset64 (p : * mut i64 , b : i64) -> u8 { let r : u8 ; asm ! (bt ! ("btrq") , "setc {r}" , p = in (reg) p , b = in (reg) b , r = out (reg_byte) r , options (nostack , att_syntax)) ; r }
}

macro_rules! _bittestandcomplement64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bittestandcomplement64 in module {}", module_path!());
    };
}

mkfn!{
    _bittestandcomplement64_introspect!();
    # [doc = " Returns the bit in position `b` of the memory addressed by `p`, then inverts that bit."] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_bittestandcomplement64)"] # [inline] # [cfg_attr (test , assert_instr (btc))] # [stable (feature = "simd_x86_bittest" , since = "1.55.0")] pub unsafe fn _bittestandcomplement64 (p : * mut i64 , b : i64) -> u8 { let r : u8 ; asm ! (bt ! ("btcq") , "setc {r}" , p = in (reg) p , b = in (reg) b , r = out (reg_byte) r , options (nostack , att_syntax)) ; r }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: x86_64 :: * ;}

macro_rules! test_bittest64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bittest64 in module {}", module_path!());
    };
}

mkfn!{
    test_bittest64_introspect!();
    # [test] # [cfg_attr (miri , ignore)] fn test_bittest64 () { unsafe { let a = 0b0101_0000i64 ; assert_eq ! (_bittest64 (& a as _ , 4) , 1) ; assert_eq ! (_bittest64 (& a as _ , 5) , 0) ; } }
}

macro_rules! test_bittestandset64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bittestandset64 in module {}", module_path!());
    };
}

mkfn!{
    test_bittestandset64_introspect!();
    # [test] # [cfg_attr (miri , ignore)] fn test_bittestandset64 () { unsafe { let mut a = 0b0101_0000i64 ; assert_eq ! (_bittestandset64 (& mut a as _ , 4) , 1) ; assert_eq ! (_bittestandset64 (& mut a as _ , 4) , 1) ; assert_eq ! (_bittestandset64 (& mut a as _ , 5) , 0) ; assert_eq ! (_bittestandset64 (& mut a as _ , 5) , 1) ; } }
}

macro_rules! test_bittestandreset64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bittestandreset64 in module {}", module_path!());
    };
}

mkfn!{
    test_bittestandreset64_introspect!();
    # [test] # [cfg_attr (miri , ignore)] fn test_bittestandreset64 () { unsafe { let mut a = 0b0101_0000i64 ; assert_eq ! (_bittestandreset64 (& mut a as _ , 4) , 1) ; assert_eq ! (_bittestandreset64 (& mut a as _ , 4) , 0) ; assert_eq ! (_bittestandreset64 (& mut a as _ , 5) , 0) ; assert_eq ! (_bittestandreset64 (& mut a as _ , 5) , 0) ; } }
}

macro_rules! test_bittestandcomplement64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bittestandcomplement64 in module {}", module_path!());
    };
}

mkfn!{
    test_bittestandcomplement64_introspect!();
    # [test] # [cfg_attr (miri , ignore)] fn test_bittestandcomplement64 () { unsafe { let mut a = 0b0101_0000i64 ; assert_eq ! (_bittestandcomplement64 (& mut a as _ , 4) , 1) ; assert_eq ! (_bittestandcomplement64 (& mut a as _ , 4) , 0) ; assert_eq ! (_bittestandcomplement64 (& mut a as _ , 4) , 1) ; assert_eq ! (_bittestandcomplement64 (& mut a as _ , 5) , 0) ; assert_eq ! (_bittestandcomplement64 (& mut a as _ , 5) , 1) ; } }
} 
            }}