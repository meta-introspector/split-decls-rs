mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{unsafe extern "unadjusted" { # [link_name = "llvm.arm.smulbb"] fn arm_smulbb (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.smulbt"] fn arm_smulbt (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.smultb"] fn arm_smultb (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.smultt"] fn arm_smultt (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.smulwb"] fn arm_smulwb (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.smulwt"] fn arm_smulwt (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.qadd"] fn arm_qadd (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.qsub"] fn arm_qsub (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.smlabb"] fn arm_smlabb (a : i32 , b : i32 , c : i32) -> i32 ; # [link_name = "llvm.arm.smlabt"] fn arm_smlabt (a : i32 , b : i32 , c : i32) -> i32 ; # [link_name = "llvm.arm.smlatb"] fn arm_smlatb (a : i32 , b : i32 , c : i32) -> i32 ; # [link_name = "llvm.arm.smlatt"] fn arm_smlatt (a : i32 , b : i32 , c : i32) -> i32 ; # [link_name = "llvm.arm.smlawb"] fn arm_smlawb (a : i32 , b : i32 , c : i32) -> i32 ; # [link_name = "llvm.arm.smlawt"] fn arm_smlawt (a : i32 , b : i32 , c : i32) -> i32 ; }}

macro_rules! __smulbb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smulbb in module {}", module_path!());
    };
}

mkfn!{
    __smulbb_introspect!();
    # [doc = " Insert a SMULBB instruction"] # [doc = ""] # [doc = " Returns the equivalent of a\\[0\\] * b\\[0\\]"] # [doc = " where \\[0\\] is the lower 16 bits and \\[1\\] is the upper 16 bits."] # [inline] # [cfg_attr (test , assert_instr (smulbb))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smulbb (a : i32 , b : i32) -> i32 { arm_smulbb (a , b) }
}

macro_rules! __smultb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smultb in module {}", module_path!());
    };
}

mkfn!{
    __smultb_introspect!();
    # [doc = " Insert a SMULTB instruction"] # [doc = ""] # [doc = " Returns the equivalent of a\\[0\\] * b\\[1\\]"] # [doc = " where \\[0\\] is the lower 16 bits and \\[1\\] is the upper 16 bits."] # [inline] # [cfg_attr (test , assert_instr (smultb))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smultb (a : i32 , b : i32) -> i32 { arm_smultb (a , b) }
}

macro_rules! __smulbt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smulbt in module {}", module_path!());
    };
}

mkfn!{
    __smulbt_introspect!();
    # [doc = " Insert a SMULTB instruction"] # [doc = ""] # [doc = " Returns the equivalent of a\\[1\\] * b\\[0\\]"] # [doc = " where \\[0\\] is the lower 16 bits and \\[1\\] is the upper 16 bits."] # [inline] # [cfg_attr (test , assert_instr (smulbt))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smulbt (a : i32 , b : i32) -> i32 { arm_smulbt (a , b) }
}

macro_rules! __smultt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smultt in module {}", module_path!());
    };
}

mkfn!{
    __smultt_introspect!();
    # [doc = " Insert a SMULTT instruction"] # [doc = ""] # [doc = " Returns the equivalent of a\\[1\\] * b\\[1\\]"] # [doc = " where \\[0\\] is the lower 16 bits and \\[1\\] is the upper 16 bits."] # [inline] # [cfg_attr (test , assert_instr (smultt))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smultt (a : i32 , b : i32) -> i32 { arm_smultt (a , b) }
}

macro_rules! __smulwb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smulwb in module {}", module_path!());
    };
}

mkfn!{
    __smulwb_introspect!();
    # [doc = " Insert a SMULWB instruction"] # [doc = ""] # [doc = " Multiplies the 32-bit signed first operand with the low halfword"] # [doc = " (as a 16-bit signed integer) of the second operand."] # [doc = " Return the top 32 bits of the 48-bit product"] # [inline] # [cfg_attr (test , assert_instr (smulwb))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smulwb (a : i32 , b : i32) -> i32 { arm_smulwb (a , b) }
}

macro_rules! __smulwt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smulwt in module {}", module_path!());
    };
}

mkfn!{
    __smulwt_introspect!();
    # [doc = " Insert a SMULWT instruction"] # [doc = ""] # [doc = " Multiplies the 32-bit signed first operand with the high halfword"] # [doc = " (as a 16-bit signed integer) of the second operand."] # [doc = " Return the top 32 bits of the 48-bit product"] # [inline] # [cfg_attr (test , assert_instr (smulwt))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smulwt (a : i32 , b : i32) -> i32 { arm_smulwt (a , b) }
}

macro_rules! __qadd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __qadd in module {}", module_path!());
    };
}

mkfn!{
    __qadd_introspect!();
    # [doc = " Signed saturating addition"] # [doc = ""] # [doc = " Returns the 32-bit saturating signed equivalent of a + b."] # [doc = " Sets the Q flag if saturation occurs."] # [inline] # [cfg_attr (test , assert_instr (qadd))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __qadd (a : i32 , b : i32) -> i32 { arm_qadd (a , b) }
}

macro_rules! __qsub_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __qsub in module {}", module_path!());
    };
}

mkfn!{
    __qsub_introspect!();
    # [doc = " Signed saturating subtraction"] # [doc = ""] # [doc = " Returns the 32-bit saturating signed equivalent of a - b."] # [doc = " Sets the Q flag if saturation occurs."] # [inline] # [cfg_attr (test , assert_instr (qsub))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __qsub (a : i32 , b : i32) -> i32 { arm_qsub (a , b) }
}

macro_rules! __qdbl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __qdbl in module {}", module_path!());
    };
}

mkfn!{
    __qdbl_introspect!();
    # [doc = " Insert a QADD instruction"] # [doc = ""] # [doc = " Returns the 32-bit saturating signed equivalent of a + a"] # [doc = " Sets the Q flag if saturation occurs."] # [inline] # [cfg_attr (test , assert_instr (qadd))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __qdbl (a : i32) -> i32 { arm_qadd (a , a) }
}

macro_rules! __smlabb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smlabb in module {}", module_path!());
    };
}

mkfn!{
    __smlabb_introspect!();
    # [doc = " Insert a SMLABB instruction"] # [doc = ""] # [doc = " Returns the equivalent of a\\[0\\] * b\\[0\\] + c"] # [doc = " where \\[0\\] is the lower 16 bits and \\[1\\] is the upper 16 bits."] # [doc = " Sets the Q flag if overflow occurs on the addition."] # [inline] # [cfg_attr (test , assert_instr (smlabb))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smlabb (a : i32 , b : i32 , c : i32) -> i32 { arm_smlabb (a , b , c) }
}

macro_rules! __smlabt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smlabt in module {}", module_path!());
    };
}

mkfn!{
    __smlabt_introspect!();
    # [doc = " Insert a SMLABT instruction"] # [doc = ""] # [doc = " Returns the equivalent of a\\[0\\] * b\\[1\\] + c"] # [doc = " where \\[0\\] is the lower 16 bits and \\[1\\] is the upper 16 bits."] # [doc = " Sets the Q flag if overflow occurs on the addition."] # [inline] # [cfg_attr (test , assert_instr (smlabt))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smlabt (a : i32 , b : i32 , c : i32) -> i32 { arm_smlabt (a , b , c) }
}

macro_rules! __smlatb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smlatb in module {}", module_path!());
    };
}

mkfn!{
    __smlatb_introspect!();
    # [doc = " Insert a SMLATB instruction"] # [doc = ""] # [doc = " Returns the equivalent of a\\[1\\] * b\\[0\\] + c"] # [doc = " where \\[0\\] is the lower 16 bits and \\[1\\] is the upper 16 bits."] # [doc = " Sets the Q flag if overflow occurs on the addition."] # [inline] # [cfg_attr (test , assert_instr (smlatb))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smlatb (a : i32 , b : i32 , c : i32) -> i32 { arm_smlatb (a , b , c) }
}

macro_rules! __smlatt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smlatt in module {}", module_path!());
    };
}

mkfn!{
    __smlatt_introspect!();
    # [doc = " Insert a SMLATT instruction"] # [doc = ""] # [doc = " Returns the equivalent of a\\[1\\] * b\\[1\\] + c"] # [doc = " where \\[0\\] is the lower 16 bits and \\[1\\] is the upper 16 bits."] # [doc = " Sets the Q flag if overflow occurs on the addition."] # [inline] # [cfg_attr (test , assert_instr (smlatt))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smlatt (a : i32 , b : i32 , c : i32) -> i32 { arm_smlatt (a , b , c) }
}

macro_rules! __smlawb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smlawb in module {}", module_path!());
    };
}

mkfn!{
    __smlawb_introspect!();
    # [doc = " Insert a SMLAWB instruction"] # [doc = ""] # [doc = " Returns the equivalent of (a * b\\[0\\] + (c << 16)) >> 16"] # [doc = " where \\[0\\] is the lower 16 bits and \\[1\\] is the upper 16 bits."] # [doc = " Sets the Q flag if overflow occurs on the addition."] # [inline] # [cfg_attr (test , assert_instr (smlawb))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smlawb (a : i32 , b : i32 , c : i32) -> i32 { arm_smlawb (a , b , c) }
}

macro_rules! __smlawt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smlawt in module {}", module_path!());
    };
}

mkfn!{
    __smlawt_introspect!();
    # [doc = " Insert a SMLAWT instruction"] # [doc = ""] # [doc = " Returns the equivalent of (a * b\\[1\\] + (c << 16)) >> 16"] # [doc = " where \\[0\\] is the lower 16 bits and \\[1\\] is the upper 16 bits."] # [doc = " Sets the Q flag if overflow occurs on the addition."] # [inline] # [cfg_attr (test , assert_instr (smlawt))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smlawt (a : i32 , b : i32 , c : i32) -> i32 { arm_smlawt (a , b , c) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: { arm :: * , simd :: { i8x4 , i16x2 , u8x4 } , } ;}
mkuse!{use std :: mem :: transmute ;}
mkuse!{use stdarch_test :: simd_test ;}

macro_rules! smulbb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smulbb in module {}", module_path!());
    };
}

mkfn!{
    smulbb_introspect!();
    # [test] fn smulbb () { unsafe { let a = i16x2 :: new (10 , 20) ; let b = i16x2 :: new (30 , 40) ; assert_eq ! (super :: __smulbb (transmute (a) , transmute (b)) , 10 * 30) ; } }
}

macro_rules! smulbt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smulbt in module {}", module_path!());
    };
}

mkfn!{
    smulbt_introspect!();
    # [test] fn smulbt () { unsafe { let a = i16x2 :: new (10 , 20) ; let b = i16x2 :: new (30 , 40) ; assert_eq ! (super :: __smulbt (transmute (a) , transmute (b)) , 10 * 40) ; } }
}

macro_rules! smultb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smultb in module {}", module_path!());
    };
}

mkfn!{
    smultb_introspect!();
    # [test] fn smultb () { unsafe { let a = i16x2 :: new (10 , 20) ; let b = i16x2 :: new (30 , 40) ; assert_eq ! (super :: __smultb (transmute (a) , transmute (b)) , 20 * 30) ; } }
}

macro_rules! smultt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smultt in module {}", module_path!());
    };
}

mkfn!{
    smultt_introspect!();
    # [test] fn smultt () { unsafe { let a = i16x2 :: new (10 , 20) ; let b = i16x2 :: new (30 , 40) ; assert_eq ! (super :: __smultt (transmute (a) , transmute (b)) , 20 * 40) ; } }
}

macro_rules! smulwb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smulwb in module {}", module_path!());
    };
}

mkfn!{
    smulwb_introspect!();
    # [test] fn smulwb () { unsafe { let a = i16x2 :: new (10 , 20) ; let b = 30 ; assert_eq ! (super :: __smulwb (transmute (a) , b) , 20 * b) ; } }
}

macro_rules! smulwt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smulwt in module {}", module_path!());
    };
}

mkfn!{
    smulwt_introspect!();
    # [test] fn smulwt () { unsafe { let a = i16x2 :: new (10 , 20) ; let b = 30 ; assert_eq ! (super :: __smulwt (transmute (a) , b) , (10 * b) >> 16) ; } }
}

macro_rules! qadd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qadd in module {}", module_path!());
    };
}

mkfn!{
    qadd_introspect!();
    # [test] fn qadd () { unsafe { assert_eq ! (super :: __qadd (- 10 , 60) , 50) ; assert_eq ! (super :: __qadd (i32 :: MAX , 10) , i32 :: MAX) ; assert_eq ! (super :: __qadd (i32 :: MIN , - 10) , i32 :: MIN) ; } }
}

macro_rules! qsub_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qsub in module {}", module_path!());
    };
}

mkfn!{
    qsub_introspect!();
    # [test] fn qsub () { unsafe { assert_eq ! (super :: __qsub (10 , 60) , - 50) ; assert_eq ! (super :: __qsub (i32 :: MAX , - 10) , i32 :: MAX) ; assert_eq ! (super :: __qsub (i32 :: MIN , 10) , i32 :: MIN) ; } }
}

macro_rules! qdbl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qdbl in module {}", module_path!());
    };
}

mkfn!{
    qdbl_introspect!();
    fn qdbl () { unsafe { assert_eq ! (super :: __qdbl (10) , 20) ; assert_eq ! (super :: __qdbl (i32 :: MAX) , i32 :: MAX) ; } }
}

macro_rules! smlabb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smlabb in module {}", module_path!());
    };
}

mkfn!{
    smlabb_introspect!();
    fn smlabb () { unsafe { let a = i16x2 :: new (10 , 20) ; let b = i16x2 :: new (30 , 40) ; let c = 50 ; let r = (10 * 30) + c ; assert_eq ! (super :: __smlabb (transmute (a) , transmute (b) , c) , r) ; } }
}

macro_rules! smlabt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smlabt in module {}", module_path!());
    };
}

mkfn!{
    smlabt_introspect!();
    fn smlabt () { unsafe { let a = i16x2 :: new (10 , 20) ; let b = i16x2 :: new (30 , 40) ; let c = 50 ; let r = (10 * 40) + c ; assert_eq ! (super :: __smlabt (transmute (a) , transmute (b) , c) , r) ; } }
}

macro_rules! smlatb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smlatb in module {}", module_path!());
    };
}

mkfn!{
    smlatb_introspect!();
    fn smlatb () { unsafe { let a = i16x2 :: new (10 , 20) ; let b = i16x2 :: new (30 , 40) ; let c = 50 ; let r = (20 * 30) + c ; assert_eq ! (super :: __smlabt (transmute (a) , transmute (b) , c) , r) ; } }
}

macro_rules! smlatt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smlatt in module {}", module_path!());
    };
}

mkfn!{
    smlatt_introspect!();
    fn smlatt () { unsafe { let a = i16x2 :: new (10 , 20) ; let b = i16x2 :: new (30 , 40) ; let c = 50 ; let r = (20 * 40) + c ; assert_eq ! (super :: __smlatt (transmute (a) , transmute (b) , c) , r) ; } }
}

macro_rules! smlawb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smlawb in module {}", module_path!());
    };
}

mkfn!{
    smlawb_introspect!();
    fn smlawb () { unsafe { let a : i32 = 10 ; let b = i16x2 :: new (30 , 40) ; let c : i32 = 50 ; let r : i32 = ((a * 30) + (c << 16)) >> 16 ; assert_eq ! (super :: __smlawb (a , transmute (b) , c) , r) ; } }
}

macro_rules! smlawt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smlawt in module {}", module_path!());
    };
}

mkfn!{
    smlawt_introspect!();
    fn smlawt () { unsafe { let a : i32 = 10 ; let b = i16x2 :: new (30 , 40) ; let c : i32 = 50 ; let r : i32 = ((a * 40) + (c << 16)) >> 16 ; assert_eq ! (super :: __smlawt (a , transmute (b) , c) , r) ; } }
} 
            }}