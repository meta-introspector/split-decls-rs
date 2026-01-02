mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{unsafe extern "unadjusted" { # [link_name = "llvm.aarch64.tstart"] fn aarch64_tstart () -> u64 ; # [link_name = "llvm.aarch64.tcommit"] fn aarch64_tcommit () ; # [link_name = "llvm.aarch64.tcancel"] fn aarch64_tcancel (imm0 : u64) ; # [link_name = "llvm.aarch64.ttest"] fn aarch64_ttest () -> u64 ; }}
mkitem!{# [doc = " Transaction successfully started."] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMSTART_SUCCESS : u64 = 0x00_u64 ;}
mkitem!{# [doc = " Extraction mask for failure reason"] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMFAILURE_REASON : u64 = 0x00007FFF_u64 ;}
mkitem!{# [doc = " Transaction retry is possible."] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMFAILURE_RTRY : u64 = 1 << 15 ;}
mkitem!{# [doc = " Transaction executed a TCANCEL instruction"] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMFAILURE_CNCL : u64 = 1 << 16 ;}
mkitem!{# [doc = " Transaction aborted because a conflict occurred"] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMFAILURE_MEM : u64 = 1 << 17 ;}
mkitem!{# [doc = " Fallback error type for any other reason"] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMFAILURE_IMP : u64 = 1 << 18 ;}
mkitem!{# [doc = " Transaction aborted because a non-permissible operation was attempted"] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMFAILURE_ERR : u64 = 1 << 19 ;}
mkitem!{# [doc = " Transaction aborted due to read or write set limit was exceeded"] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMFAILURE_SIZE : u64 = 1 << 20 ;}
mkitem!{# [doc = " Transaction aborted due to transactional nesting level was exceeded"] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMFAILURE_NEST : u64 = 1 << 21 ;}
mkitem!{# [doc = " Transaction aborted due to a debug trap."] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMFAILURE_DBG : u64 = 1 << 22 ;}
mkitem!{# [doc = " Transaction failed from interrupt"] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMFAILURE_INT : u64 = 1 << 23 ;}
mkitem!{# [doc = " Indicates a TRIVIAL version of TM is available"] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub const _TMFAILURE_TRIVIAL : u64 = 1 << 24 ;}

macro_rules! __tstart_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __tstart in module {}", module_path!());
    };
}

mkfn!{
    __tstart_introspect!();
    # [doc = " Starts a new transaction. When the transaction starts successfully the return value is 0."] # [doc = " If the transaction fails, all state modifications are discarded and a cause of the failure"] # [doc = " is encoded in the return value."] # [doc = ""] # [doc = " [ARM TME Intrinsics](https://developer.arm.com/docs/101028/0010/transactional-memory-extension-tme-intrinsics)."] # [inline] # [target_feature (enable = "tme")] # [cfg_attr (all (test , not (target_env = "msvc")) , assert_instr (tstart))] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub unsafe fn __tstart () -> u64 { aarch64_tstart () }
}

macro_rules! __tcommit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __tcommit in module {}", module_path!());
    };
}

mkfn!{
    __tcommit_introspect!();
    # [doc = " Commits the current transaction. For a nested transaction, the only effect is that the"] # [doc = " transactional nesting depth is decreased. For an outer transaction, the state modifications"] # [doc = " performed transactionally are committed to the architectural state."] # [doc = ""] # [doc = " [ARM TME Intrinsics](https://developer.arm.com/docs/101028/0010/transactional-memory-extension-tme-intrinsics)."] # [inline] # [target_feature (enable = "tme")] # [cfg_attr (all (test , not (target_env = "msvc")) , assert_instr (tcommit))] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub unsafe fn __tcommit () { aarch64_tcommit () }
}

macro_rules! __tcancel_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __tcancel in module {}", module_path!());
    };
}

mkfn!{
    __tcancel_introspect!();
    # [doc = " Cancels the current transaction and discards all state modifications that were performed transactionally."] # [doc = ""] # [doc = " [ARM TME Intrinsics](https://developer.arm.com/docs/101028/0010/transactional-memory-extension-tme-intrinsics)."] # [inline] # [target_feature (enable = "tme")] # [cfg_attr (all (test , not (target_env = "msvc")) , assert_instr (tcancel , IMM16 = 0x0))] # [rustc_legacy_const_generics (0)] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub unsafe fn __tcancel < const IMM16 : u64 > () { static_assert ! (IMM16 <= 65535) ; aarch64_tcancel (IMM16) ; }
}

macro_rules! __ttest_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __ttest in module {}", module_path!());
    };
}

mkfn!{
    __ttest_introspect!();
    # [doc = " Tests if executing inside a transaction. If no transaction is currently executing,"] # [doc = " the return value is 0. Otherwise, this intrinsic returns the depth of the transaction."] # [doc = ""] # [doc = " [ARM TME Intrinsics](https://developer.arm.com/docs/101028/0010/transactional-memory-extension-tme-intrinsics)."] # [inline] # [target_feature (enable = "tme")] # [cfg_attr (all (test , not (target_env = "msvc")) , assert_instr (ttest))] # [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub unsafe fn __ttest () -> u64 { aarch64_ttest () }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use stdarch_test :: simd_test ;}
mkuse!{use crate :: core_arch :: aarch64 :: * ;}
mkitem!{const CANCEL_CODE : u64 = (0 | (0x123 & _TMFAILURE_REASON) as u64) as u64 ;}

macro_rules! test_tstart_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tstart in module {}", module_path!());
    };
}

mkfn!{
    test_tstart_introspect!();
    # [simd_test (enable = "tme")] unsafe fn test_tstart () { let mut x = 0 ; for i in 0 .. 10 { let code = tme :: __tstart () ; if code == _TMSTART_SUCCESS { x += 1 ; assert_eq ! (x , i + 1) ; break ; } assert_eq ! (x , 0) ; } }
}

macro_rules! test_tcommit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tcommit in module {}", module_path!());
    };
}

mkfn!{
    test_tcommit_introspect!();
    # [simd_test (enable = "tme")] unsafe fn test_tcommit () { let mut x = 0 ; for i in 0 .. 10 { let code = tme :: __tstart () ; if code == _TMSTART_SUCCESS { x += 1 ; assert_eq ! (x , i + 1) ; tme :: __tcommit () ; } assert_eq ! (x , i + 1) ; } }
}

macro_rules! test_tcancel_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tcancel in module {}", module_path!());
    };
}

mkfn!{
    test_tcancel_introspect!();
    # [simd_test (enable = "tme")] unsafe fn test_tcancel () { let mut x = 0 ; for i in 0 .. 10 { let code = tme :: __tstart () ; if code == _TMSTART_SUCCESS { x += 1 ; assert_eq ! (x , i + 1) ; tme :: __tcancel :: < CANCEL_CODE > () ; break ; } } assert_eq ! (x , 0) ; }
}

macro_rules! test_ttest_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_ttest in module {}", module_path!());
    };
}

mkfn!{
    test_ttest_introspect!();
    # [simd_test (enable = "tme")] unsafe fn test_ttest () { for _ in 0 .. 10 { let code = tme :: __tstart () ; if code == _TMSTART_SUCCESS { if tme :: __ttest () == 2 { tme :: __tcancel :: < CANCEL_CODE > () ; break ; } } } }
} 
            }}