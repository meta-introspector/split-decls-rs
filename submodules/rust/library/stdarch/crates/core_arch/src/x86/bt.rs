mkuse!{use crate :: arch :: asm ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [cfg (target_pointer_width = "32")] macro_rules ! bt { ($ inst : expr) => { concat ! ($ inst , " {b:e}, ({p:e})") } ; }}
mkitem!{# [cfg (target_pointer_width = "64")] macro_rules ! bt { ($ inst : expr) => { concat ! ($ inst , " {b:e}, ({p})") } ; }}

macro_rules! _bittest_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bittest in module {}", module_path!());
    };
}

mkfn!{
    _bittest_introspect!();
    # [doc = " Returns the bit in position `b` of the memory addressed by `p`."] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_bittest)"] # [inline] # [cfg_attr (test , assert_instr (bt))] # [stable (feature = "simd_x86_bittest" , since = "1.55.0")] pub unsafe fn _bittest (p : * const i32 , b : i32) -> u8 { let r : u8 ; asm ! (bt ! ("btl") , "setc {r}" , p = in (reg) p , b = in (reg) b , r = out (reg_byte) r , options (readonly , nostack , pure , att_syntax)) ; r }
}

macro_rules! _bittestandset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bittestandset in module {}", module_path!());
    };
}

mkfn!{
    _bittestandset_introspect!();
    # [doc = " Returns the bit in position `b` of the memory addressed by `p`, then sets the bit to `1`."] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_bittestandset)"] # [inline] # [cfg_attr (test , assert_instr (bts))] # [stable (feature = "simd_x86_bittest" , since = "1.55.0")] pub unsafe fn _bittestandset (p : * mut i32 , b : i32) -> u8 { let r : u8 ; asm ! (bt ! ("btsl") , "setc {r}" , p = in (reg) p , b = in (reg) b , r = out (reg_byte) r , options (nostack , att_syntax)) ; r }
}

macro_rules! _bittestandreset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bittestandreset in module {}", module_path!());
    };
}

mkfn!{
    _bittestandreset_introspect!();
    # [doc = " Returns the bit in position `b` of the memory addressed by `p`, then resets that bit to `0`."] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_bittestandreset)"] # [inline] # [cfg_attr (test , assert_instr (btr))] # [stable (feature = "simd_x86_bittest" , since = "1.55.0")] pub unsafe fn _bittestandreset (p : * mut i32 , b : i32) -> u8 { let r : u8 ; asm ! (bt ! ("btrl") , "setc {r}" , p = in (reg) p , b = in (reg) b , r = out (reg_byte) r , options (nostack , att_syntax)) ; r }
}

macro_rules! _bittestandcomplement_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _bittestandcomplement in module {}", module_path!());
    };
}

mkfn!{
    _bittestandcomplement_introspect!();
    # [doc = " Returns the bit in position `b` of the memory addressed by `p`, then inverts that bit."] # [doc = ""] # [doc = " [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_bittestandcomplement)"] # [inline] # [cfg_attr (test , assert_instr (btc))] # [stable (feature = "simd_x86_bittest" , since = "1.55.0")] pub unsafe fn _bittestandcomplement (p : * mut i32 , b : i32) -> u8 { let r : u8 ; asm ! (bt ! ("btcl") , "setc {r}" , p = in (reg) p , b = in (reg) b , r = out (reg_byte) r , options (nostack , att_syntax)) ; r }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: x86 :: * ;}

macro_rules! test_bittest_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bittest in module {}", module_path!());
    };
}

mkfn!{
    test_bittest_introspect!();
    # [test] # [cfg_attr (miri , ignore)] fn test_bittest () { unsafe { let a = 0b0101_0000i32 ; assert_eq ! (_bittest (& a as _ , 4) , 1) ; assert_eq ! (_bittest (& a as _ , 5) , 0) ; } }
}

macro_rules! test_bittestandset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bittestandset in module {}", module_path!());
    };
}

mkfn!{
    test_bittestandset_introspect!();
    # [test] # [cfg_attr (miri , ignore)] fn test_bittestandset () { unsafe { let mut a = 0b0101_0000i32 ; assert_eq ! (_bittestandset (& mut a as _ , 4) , 1) ; assert_eq ! (_bittestandset (& mut a as _ , 4) , 1) ; assert_eq ! (_bittestandset (& mut a as _ , 5) , 0) ; assert_eq ! (_bittestandset (& mut a as _ , 5) , 1) ; } }
}

macro_rules! test_bittestandreset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bittestandreset in module {}", module_path!());
    };
}

mkfn!{
    test_bittestandreset_introspect!();
    # [test] # [cfg_attr (miri , ignore)] fn test_bittestandreset () { unsafe { let mut a = 0b0101_0000i32 ; assert_eq ! (_bittestandreset (& mut a as _ , 4) , 1) ; assert_eq ! (_bittestandreset (& mut a as _ , 4) , 0) ; assert_eq ! (_bittestandreset (& mut a as _ , 5) , 0) ; assert_eq ! (_bittestandreset (& mut a as _ , 5) , 0) ; } }
}

macro_rules! test_bittestandcomplement_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bittestandcomplement in module {}", module_path!());
    };
}

mkfn!{
    test_bittestandcomplement_introspect!();
    # [test] # [cfg_attr (miri , ignore)] fn test_bittestandcomplement () { unsafe { let mut a = 0b0101_0000i32 ; assert_eq ! (_bittestandcomplement (& mut a as _ , 4) , 1) ; assert_eq ! (_bittestandcomplement (& mut a as _ , 4) , 0) ; assert_eq ! (_bittestandcomplement (& mut a as _ , 4) , 1) ; assert_eq ! (_bittestandcomplement (& mut a as _ , 5) , 0) ; assert_eq ! (_bittestandcomplement (& mut a as _ , 5) , 1) ; } }
} 
            }}