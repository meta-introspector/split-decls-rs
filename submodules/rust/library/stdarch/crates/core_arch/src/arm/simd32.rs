mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkuse!{use crate :: mem :: transmute ;}
mkitem!{# [doc = " ARM-specific vector of four packed `i8` packed into a 32-bit integer."] # [allow (non_camel_case_types)] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub type int8x4_t = i32 ;}
mkitem!{# [doc = " ARM-specific vector of four packed `u8` packed into a 32-bit integer."] # [allow (non_camel_case_types)] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub type uint8x4_t = u32 ;}
mkitem!{# [doc = " ARM-specific vector of two packed `i16` packed into a 32-bit integer."] # [allow (non_camel_case_types)] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub type int16x2_t = i32 ;}
mkitem!{# [doc = " ARM-specific vector of two packed `u16` packed into a 32-bit integer."] # [allow (non_camel_case_types)] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub type uint16x2_t = u32 ;}
mkitem!{macro_rules ! dsp_call { ($ name : expr , $ a : expr , $ b : expr) => { transmute ($ name (transmute ($ a) , transmute ($ b))) } ; }}
mkitem!{unsafe extern "unadjusted" { # [link_name = "llvm.arm.qadd8"] fn arm_qadd8 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.qsub8"] fn arm_qsub8 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.qsub16"] fn arm_qsub16 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.qadd16"] fn arm_qadd16 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.qasx"] fn arm_qasx (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.qsax"] fn arm_qsax (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.sadd16"] fn arm_sadd16 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.sadd8"] fn arm_sadd8 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.smlad"] fn arm_smlad (a : i32 , b : i32 , c : i32) -> i32 ; # [link_name = "llvm.arm.smlsd"] fn arm_smlsd (a : i32 , b : i32 , c : i32) -> i32 ; # [link_name = "llvm.arm.sasx"] fn arm_sasx (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.sel"] fn arm_sel (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.shadd8"] fn arm_shadd8 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.shadd16"] fn arm_shadd16 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.shsub8"] fn arm_shsub8 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.ssub8"] fn arm_ssub8 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.usub8"] fn arm_usub8 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.shsub16"] fn arm_shsub16 (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.smuad"] fn arm_smuad (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.smuadx"] fn arm_smuadx (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.smusd"] fn arm_smusd (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.smusdx"] fn arm_smusdx (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.arm.usad8"] fn arm_usad8 (a : i32 , b : i32) -> u32 ; }}

macro_rules! __qadd8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __qadd8 in module {}", module_path!());
    };
}

mkfn!{
    __qadd8_introspect!();
    # [doc = " Saturating four 8-bit integer additions"] # [doc = ""] # [doc = " Returns the 8-bit signed equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = a\\[0\\] + b\\[0\\]"] # [doc = " res\\[1\\] = a\\[1\\] + b\\[1\\]"] # [doc = " res\\[2\\] = a\\[2\\] + b\\[2\\]"] # [doc = " res\\[3\\] = a\\[3\\] + b\\[3\\]"] # [inline] # [cfg_attr (test , assert_instr (qadd8))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __qadd8 (a : int8x4_t , b : int8x4_t) -> int8x4_t { dsp_call ! (arm_qadd8 , a , b) }
}

macro_rules! __qsub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __qsub8 in module {}", module_path!());
    };
}

mkfn!{
    __qsub8_introspect!();
    # [doc = " Saturating two 8-bit integer subtraction"] # [doc = ""] # [doc = " Returns the 8-bit signed equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = a\\[0\\] - b\\[0\\]"] # [doc = " res\\[1\\] = a\\[1\\] - b\\[1\\]"] # [doc = " res\\[2\\] = a\\[2\\] - b\\[2\\]"] # [doc = " res\\[3\\] = a\\[3\\] - b\\[3\\]"] # [inline] # [cfg_attr (test , assert_instr (qsub8))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __qsub8 (a : int8x4_t , b : int8x4_t) -> int8x4_t { dsp_call ! (arm_qsub8 , a , b) }
}

macro_rules! __qsub16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __qsub16 in module {}", module_path!());
    };
}

mkfn!{
    __qsub16_introspect!();
    # [doc = " Saturating two 16-bit integer subtraction"] # [doc = ""] # [doc = " Returns the 16-bit signed equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = a\\[0\\] - b\\[0\\]"] # [doc = " res\\[1\\] = a\\[1\\] - b\\[1\\]"] # [inline] # [cfg_attr (test , assert_instr (qsub16))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __qsub16 (a : int16x2_t , b : int16x2_t) -> int16x2_t { dsp_call ! (arm_qsub16 , a , b) }
}

macro_rules! __qadd16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __qadd16 in module {}", module_path!());
    };
}

mkfn!{
    __qadd16_introspect!();
    # [doc = " Saturating two 16-bit integer additions"] # [doc = ""] # [doc = " Returns the 16-bit signed equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = a\\[0\\] + b\\[0\\]"] # [doc = " res\\[1\\] = a\\[1\\] + b\\[1\\]"] # [inline] # [cfg_attr (test , assert_instr (qadd16))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __qadd16 (a : int16x2_t , b : int16x2_t) -> int16x2_t { dsp_call ! (arm_qadd16 , a , b) }
}

macro_rules! __qasx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __qasx in module {}", module_path!());
    };
}

mkfn!{
    __qasx_introspect!();
    # [doc = " Returns the 16-bit signed saturated equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = a\\[0\\] - b\\[1\\]"] # [doc = " res\\[1\\] = a\\[1\\] + b\\[0\\]"] # [inline] # [cfg_attr (test , assert_instr (qasx))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __qasx (a : int16x2_t , b : int16x2_t) -> int16x2_t { dsp_call ! (arm_qasx , a , b) }
}

macro_rules! __qsax_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __qsax in module {}", module_path!());
    };
}

mkfn!{
    __qsax_introspect!();
    # [doc = " Returns the 16-bit signed saturated equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = a\\[0\\] + b\\[1\\]"] # [doc = " res\\[1\\] = a\\[1\\] - b\\[0\\]"] # [inline] # [cfg_attr (test , assert_instr (qsax))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __qsax (a : int16x2_t , b : int16x2_t) -> int16x2_t { dsp_call ! (arm_qsax , a , b) }
}

macro_rules! __sadd16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __sadd16 in module {}", module_path!());
    };
}

mkfn!{
    __sadd16_introspect!();
    # [doc = " Returns the 16-bit signed saturated equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = a\\[0\\] + b\\[1\\]"] # [doc = " res\\[1\\] = a\\[1\\] + b\\[0\\]"] # [doc = ""] # [doc = " and the GE bits of the APSR are set."] # [inline] # [cfg_attr (test , assert_instr (sadd16))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __sadd16 (a : int16x2_t , b : int16x2_t) -> int16x2_t { dsp_call ! (arm_sadd16 , a , b) }
}

macro_rules! __sadd8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __sadd8 in module {}", module_path!());
    };
}

mkfn!{
    __sadd8_introspect!();
    # [doc = " Returns the 8-bit signed saturated equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = a\\[0\\] + b\\[1\\]"] # [doc = " res\\[1\\] = a\\[1\\] + b\\[0\\]"] # [doc = " res\\[2\\] = a\\[2\\] + b\\[2\\]"] # [doc = " res\\[3\\] = a\\[3\\] + b\\[3\\]"] # [doc = ""] # [doc = " and the GE bits of the APSR are set."] # [inline] # [cfg_attr (test , assert_instr (sadd8))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __sadd8 (a : int8x4_t , b : int8x4_t) -> int8x4_t { dsp_call ! (arm_sadd8 , a , b) }
}

macro_rules! __smlad_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smlad in module {}", module_path!());
    };
}

mkfn!{
    __smlad_introspect!();
    # [doc = " Dual 16-bit Signed Multiply with Addition of products"] # [doc = " and 32-bit accumulation."] # [doc = ""] # [doc = " Returns the 16-bit signed equivalent of"] # [doc = " res = a\\[0\\] * b\\[0\\] + a\\[1\\] * b\\[1\\] + c"] # [inline] # [cfg_attr (test , assert_instr (smlad))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smlad (a : int16x2_t , b : int16x2_t , c : i32) -> i32 { arm_smlad (transmute (a) , transmute (b) , c) }
}

macro_rules! __smlsd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smlsd in module {}", module_path!());
    };
}

mkfn!{
    __smlsd_introspect!();
    # [doc = " Dual 16-bit Signed Multiply with Subtraction  of products"] # [doc = " and 32-bit accumulation and overflow detection."] # [doc = ""] # [doc = " Returns the 16-bit signed equivalent of"] # [doc = " res = a\\[0\\] * b\\[0\\] - a\\[1\\] * b\\[1\\] + c"] # [inline] # [cfg_attr (test , assert_instr (smlsd))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smlsd (a : int16x2_t , b : int16x2_t , c : i32) -> i32 { arm_smlsd (transmute (a) , transmute (b) , c) }
}

macro_rules! __sasx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __sasx in module {}", module_path!());
    };
}

mkfn!{
    __sasx_introspect!();
    # [doc = " Returns the 16-bit signed equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = a\\[0\\] - b\\[1\\]"] # [doc = " res\\[1\\] = a\\[1\\] + b\\[0\\]"] # [doc = ""] # [doc = " and the GE bits of the APSR are set."] # [inline] # [cfg_attr (test , assert_instr (sasx))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __sasx (a : int16x2_t , b : int16x2_t) -> int16x2_t { dsp_call ! (arm_sasx , a , b) }
}

macro_rules! __sel_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __sel in module {}", module_path!());
    };
}

mkfn!{
    __sel_introspect!();
    # [doc = " Select bytes from each operand according to APSR GE flags"] # [doc = ""] # [doc = " Returns the equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = GE\\[0\\] ? a\\[0\\] : b\\[0\\]"] # [doc = " res\\[1\\] = GE\\[1\\] ? a\\[1\\] : b\\[1\\]"] # [doc = " res\\[2\\] = GE\\[2\\] ? a\\[2\\] : b\\[2\\]"] # [doc = " res\\[3\\] = GE\\[3\\] ? a\\[3\\] : b\\[3\\]"] # [doc = ""] # [doc = " where GE are bits of APSR"] # [inline] # [cfg_attr (test , assert_instr (sel))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __sel (a : int8x4_t , b : int8x4_t) -> int8x4_t { dsp_call ! (arm_sel , a , b) }
}

macro_rules! __shadd8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __shadd8 in module {}", module_path!());
    };
}

mkfn!{
    __shadd8_introspect!();
    # [doc = " Signed halving parallel byte-wise addition."] # [doc = ""] # [doc = " Returns the 8-bit signed equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = (a\\[0\\] + b\\[0\\]) / 2"] # [doc = " res\\[1\\] = (a\\[1\\] + b\\[1\\]) / 2"] # [doc = " res\\[2\\] = (a\\[2\\] + b\\[2\\]) / 2"] # [doc = " res\\[3\\] = (a\\[3\\] + b\\[3\\]) / 2"] # [inline] # [cfg_attr (test , assert_instr (shadd8))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __shadd8 (a : int8x4_t , b : int8x4_t) -> int8x4_t { dsp_call ! (arm_shadd8 , a , b) }
}

macro_rules! __shadd16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __shadd16 in module {}", module_path!());
    };
}

mkfn!{
    __shadd16_introspect!();
    # [doc = " Signed halving parallel halfword-wise addition."] # [doc = ""] # [doc = " Returns the 16-bit signed equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = (a\\[0\\] + b\\[0\\]) / 2"] # [doc = " res\\[1\\] = (a\\[1\\] + b\\[1\\]) / 2"] # [inline] # [cfg_attr (test , assert_instr (shadd16))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __shadd16 (a : int16x2_t , b : int16x2_t) -> int16x2_t { dsp_call ! (arm_shadd16 , a , b) }
}

macro_rules! __shsub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __shsub8 in module {}", module_path!());
    };
}

mkfn!{
    __shsub8_introspect!();
    # [doc = " Signed halving parallel byte-wise subtraction."] # [doc = ""] # [doc = " Returns the 8-bit signed equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = (a\\[0\\] - b\\[0\\]) / 2"] # [doc = " res\\[1\\] = (a\\[1\\] - b\\[1\\]) / 2"] # [doc = " res\\[2\\] = (a\\[2\\] - b\\[2\\]) / 2"] # [doc = " res\\[3\\] = (a\\[3\\] - b\\[3\\]) / 2"] # [inline] # [cfg_attr (test , assert_instr (shsub8))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __shsub8 (a : int8x4_t , b : int8x4_t) -> int8x4_t { dsp_call ! (arm_shsub8 , a , b) }
}

macro_rules! __usub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __usub8 in module {}", module_path!());
    };
}

mkfn!{
    __usub8_introspect!();
    # [doc = " Inserts a `USUB8` instruction."] # [doc = ""] # [doc = " Returns the 8-bit unsigned equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = a\\[0\\] - a\\[0\\]"] # [doc = " res\\[1\\] = a\\[1\\] - a\\[1\\]"] # [doc = " res\\[2\\] = a\\[2\\] - a\\[2\\]"] # [doc = " res\\[3\\] = a\\[3\\] - a\\[3\\]"] # [doc = ""] # [doc = " where \\[0\\] is the lower 8 bits and \\[3\\] is the upper 8 bits."] # [doc = " The GE bits of the APSR are set."] # [inline] # [cfg_attr (test , assert_instr (usub8))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __usub8 (a : uint8x4_t , b : uint8x4_t) -> uint8x4_t { dsp_call ! (arm_usub8 , a , b) }
}

macro_rules! __ssub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __ssub8 in module {}", module_path!());
    };
}

mkfn!{
    __ssub8_introspect!();
    # [doc = " Inserts a `SSUB8` instruction."] # [doc = ""] # [doc = " Returns the 8-bit signed equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = a\\[0\\] - a\\[0\\]"] # [doc = " res\\[1\\] = a\\[1\\] - a\\[1\\]"] # [doc = " res\\[2\\] = a\\[2\\] - a\\[2\\]"] # [doc = " res\\[3\\] = a\\[3\\] - a\\[3\\]"] # [doc = ""] # [doc = " where \\[0\\] is the lower 8 bits and \\[3\\] is the upper 8 bits."] # [doc = " The GE bits of the APSR are set."] # [inline] # [cfg_attr (test , assert_instr (ssub8))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __ssub8 (a : int8x4_t , b : int8x4_t) -> int8x4_t { dsp_call ! (arm_ssub8 , a , b) }
}

macro_rules! __shsub16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __shsub16 in module {}", module_path!());
    };
}

mkfn!{
    __shsub16_introspect!();
    # [doc = " Signed halving parallel halfword-wise subtraction."] # [doc = ""] # [doc = " Returns the 16-bit signed equivalent of"] # [doc = ""] # [doc = " res\\[0\\] = (a\\[0\\] - b\\[0\\]) / 2"] # [doc = " res\\[1\\] = (a\\[1\\] - b\\[1\\]) / 2"] # [inline] # [cfg_attr (test , assert_instr (shsub16))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __shsub16 (a : int16x2_t , b : int16x2_t) -> int16x2_t { dsp_call ! (arm_shsub16 , a , b) }
}

macro_rules! __smuad_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smuad in module {}", module_path!());
    };
}

mkfn!{
    __smuad_introspect!();
    # [doc = " Signed Dual Multiply Add."] # [doc = ""] # [doc = " Returns the equivalent of"] # [doc = ""] # [doc = " res = a\\[0\\] * b\\[0\\] + a\\[1\\] * b\\[1\\]"] # [doc = ""] # [doc = " and sets the Q flag if overflow occurs on the addition."] # [inline] # [cfg_attr (test , assert_instr (smuad))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smuad (a : int16x2_t , b : int16x2_t) -> i32 { arm_smuad (transmute (a) , transmute (b)) }
}

macro_rules! __smuadx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smuadx in module {}", module_path!());
    };
}

mkfn!{
    __smuadx_introspect!();
    # [doc = " Signed Dual Multiply Add Reversed."] # [doc = ""] # [doc = " Returns the equivalent of"] # [doc = ""] # [doc = " res = a\\[0\\] * b\\[1\\] + a\\[1\\] * b\\[0\\]"] # [doc = ""] # [doc = " and sets the Q flag if overflow occurs on the addition."] # [inline] # [cfg_attr (test , assert_instr (smuadx))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smuadx (a : int16x2_t , b : int16x2_t) -> i32 { arm_smuadx (transmute (a) , transmute (b)) }
}

macro_rules! __smusd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smusd in module {}", module_path!());
    };
}

mkfn!{
    __smusd_introspect!();
    # [doc = " Signed Dual Multiply Subtract."] # [doc = ""] # [doc = " Returns the equivalent of"] # [doc = ""] # [doc = " res = a\\[0\\] * b\\[0\\] - a\\[1\\] * b\\[1\\]"] # [doc = ""] # [doc = " and sets the Q flag if overflow occurs on the addition."] # [inline] # [cfg_attr (test , assert_instr (smusd))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smusd (a : int16x2_t , b : int16x2_t) -> i32 { arm_smusd (transmute (a) , transmute (b)) }
}

macro_rules! __smusdx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __smusdx in module {}", module_path!());
    };
}

mkfn!{
    __smusdx_introspect!();
    # [doc = " Signed Dual Multiply Subtract Reversed."] # [doc = ""] # [doc = " Returns the equivalent of"] # [doc = ""] # [doc = " res = a\\[0\\] * b\\[1\\] - a\\[1\\] * b\\[0\\]"] # [doc = ""] # [doc = " and sets the Q flag if overflow occurs on the addition."] # [inline] # [cfg_attr (test , assert_instr (smusdx))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __smusdx (a : int16x2_t , b : int16x2_t) -> i32 { arm_smusdx (transmute (a) , transmute (b)) }
}

macro_rules! __usad8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __usad8 in module {}", module_path!());
    };
}

mkfn!{
    __usad8_introspect!();
    # [doc = " Sum of 8-bit absolute differences."] # [doc = ""] # [doc = " Returns the 8-bit unsigned equivalent of"] # [doc = ""] # [doc = " res = abs(a\\[0\\] - b\\[0\\]) + abs(a\\[1\\] - b\\[1\\]) +\\"] # [doc = "          (a\\[2\\] - b\\[2\\]) + (a\\[3\\] - b\\[3\\])"] # [inline] # [cfg_attr (test , assert_instr (usad8))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __usad8 (a : int8x4_t , b : int8x4_t) -> u32 { arm_usad8 (transmute (a) , transmute (b)) }
}

macro_rules! __usada8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __usada8 in module {}", module_path!());
    };
}

mkfn!{
    __usada8_introspect!();
    # [doc = " Sum of 8-bit absolute differences and constant."] # [doc = ""] # [doc = " Returns the 8-bit unsigned equivalent of"] # [doc = ""] # [doc = " res = abs(a\\[0\\] - b\\[0\\]) + abs(a\\[1\\] - b\\[1\\]) +\\"] # [doc = "          (a\\[2\\] - b\\[2\\]) + (a\\[3\\] - b\\[3\\]) + c"] # [inline] # [cfg_attr (test , assert_instr (usad8))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub unsafe fn __usada8 (a : int8x4_t , b : int8x4_t , c : u32) -> u32 { __usad8 (a , b) + c }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: simd :: { i8x4 , i16x2 , u8x4 } ;}
mkuse!{use std :: mem :: transmute ;}
mkuse!{use stdarch_test :: simd_test ;}

macro_rules! qadd8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qadd8 in module {}", module_path!());
    };
}

mkfn!{
    qadd8_introspect!();
    # [test] fn qadd8 () { unsafe { let a = i8x4 :: new (1 , 2 , 3 , i8 :: MAX) ; let b = i8x4 :: new (2 , - 1 , 0 , 1) ; let c = i8x4 :: new (3 , 1 , 3 , i8 :: MAX) ; let r : i8x4 = dsp_call ! (super :: __qadd8 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! qsub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qsub8 in module {}", module_path!());
    };
}

mkfn!{
    qsub8_introspect!();
    # [test] fn qsub8 () { unsafe { let a = i8x4 :: new (1 , 2 , 3 , i8 :: MIN) ; let b = i8x4 :: new (2 , - 1 , 0 , 1) ; let c = i8x4 :: new (- 1 , 3 , 3 , i8 :: MIN) ; let r : i8x4 = dsp_call ! (super :: __qsub8 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! qadd16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qadd16 in module {}", module_path!());
    };
}

mkfn!{
    qadd16_introspect!();
    # [test] fn qadd16 () { unsafe { let a = i16x2 :: new (1 , 2) ; let b = i16x2 :: new (2 , - 1) ; let c = i16x2 :: new (3 , 1) ; let r : i16x2 = dsp_call ! (super :: __qadd16 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! qsub16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qsub16 in module {}", module_path!());
    };
}

mkfn!{
    qsub16_introspect!();
    # [test] fn qsub16 () { unsafe { let a = i16x2 :: new (10 , 20) ; let b = i16x2 :: new (20 , - 10) ; let c = i16x2 :: new (- 10 , 30) ; let r : i16x2 = dsp_call ! (super :: __qsub16 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! qasx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qasx in module {}", module_path!());
    };
}

mkfn!{
    qasx_introspect!();
    # [test] fn qasx () { unsafe { let a = i16x2 :: new (1 , i16 :: MAX) ; let b = i16x2 :: new (2 , 2) ; let c = i16x2 :: new (- 1 , i16 :: MAX) ; let r : i16x2 = dsp_call ! (super :: __qasx , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! qsax_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function qsax in module {}", module_path!());
    };
}

mkfn!{
    qsax_introspect!();
    # [test] fn qsax () { unsafe { let a = i16x2 :: new (1 , i16 :: MAX) ; let b = i16x2 :: new (2 , 2) ; let c = i16x2 :: new (3 , i16 :: MAX - 2) ; let r : i16x2 = dsp_call ! (super :: __qsax , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! sadd16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sadd16 in module {}", module_path!());
    };
}

mkfn!{
    sadd16_introspect!();
    # [test] fn sadd16 () { unsafe { let a = i16x2 :: new (1 , i16 :: MAX) ; let b = i16x2 :: new (2 , 2) ; let c = i16x2 :: new (3 , - i16 :: MAX) ; let r : i16x2 = dsp_call ! (super :: __sadd16 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! sadd8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sadd8 in module {}", module_path!());
    };
}

mkfn!{
    sadd8_introspect!();
    # [test] fn sadd8 () { unsafe { let a = i8x4 :: new (1 , 2 , 3 , i8 :: MAX) ; let b = i8x4 :: new (4 , 3 , 2 , 2) ; let c = i8x4 :: new (5 , 5 , 5 , - i8 :: MAX) ; let r : i8x4 = dsp_call ! (super :: __sadd8 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! sasx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sasx in module {}", module_path!());
    };
}

mkfn!{
    sasx_introspect!();
    # [test] fn sasx () { unsafe { let a = i16x2 :: new (1 , 2) ; let b = i16x2 :: new (2 , 1) ; let c = i16x2 :: new (0 , 4) ; let r : i16x2 = dsp_call ! (super :: __sasx , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! smlad_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smlad in module {}", module_path!());
    };
}

mkfn!{
    smlad_introspect!();
    # [test] fn smlad () { unsafe { let a = i16x2 :: new (1 , 2) ; let b = i16x2 :: new (3 , 4) ; let r = super :: __smlad (transmute (a) , transmute (b) , 10) ; assert_eq ! (r , (1 * 3) + (2 * 4) + 10) ; } }
}

macro_rules! smlsd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smlsd in module {}", module_path!());
    };
}

mkfn!{
    smlsd_introspect!();
    # [test] fn smlsd () { unsafe { let a = i16x2 :: new (1 , 2) ; let b = i16x2 :: new (3 , 4) ; let r = super :: __smlsd (transmute (a) , transmute (b) , 10) ; assert_eq ! (r , ((1 * 3) - (2 * 4)) + 10) ; } }
}

macro_rules! sel_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sel in module {}", module_path!());
    };
}

mkfn!{
    sel_introspect!();
    # [test] fn sel () { unsafe { let a = i8x4 :: new (1 , 2 , 3 , i8 :: MAX) ; let b = i8x4 :: new (4 , 3 , 2 , 2) ; super :: __sadd8 (transmute (a) , transmute (b)) ; let c = i8x4 :: new (1 , 2 , 3 , i8 :: MAX) ; let r : i8x4 = dsp_call ! (super :: __sel , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! shadd8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function shadd8 in module {}", module_path!());
    };
}

mkfn!{
    shadd8_introspect!();
    # [test] fn shadd8 () { unsafe { let a = i8x4 :: new (1 , 2 , 3 , 4) ; let b = i8x4 :: new (5 , 4 , 3 , 2) ; let c = i8x4 :: new (3 , 3 , 3 , 3) ; let r : i8x4 = dsp_call ! (super :: __shadd8 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! shadd16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function shadd16 in module {}", module_path!());
    };
}

mkfn!{
    shadd16_introspect!();
    # [test] fn shadd16 () { unsafe { let a = i16x2 :: new (1 , 2) ; let b = i16x2 :: new (5 , 4) ; let c = i16x2 :: new (3 , 3) ; let r : i16x2 = dsp_call ! (super :: __shadd16 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! shsub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function shsub8 in module {}", module_path!());
    };
}

mkfn!{
    shsub8_introspect!();
    # [test] fn shsub8 () { unsafe { let a = i8x4 :: new (1 , 2 , 3 , 4) ; let b = i8x4 :: new (5 , 4 , 3 , 2) ; let c = i8x4 :: new (- 2 , - 1 , 0 , 1) ; let r : i8x4 = dsp_call ! (super :: __shsub8 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! ssub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ssub8 in module {}", module_path!());
    };
}

mkfn!{
    ssub8_introspect!();
    # [test] fn ssub8 () { unsafe { let a = i8x4 :: new (1 , 2 , 3 , 4) ; let b = i8x4 :: new (5 , 4 , 3 , 2) ; let c = i8x4 :: new (- 4 , - 2 , 0 , 2) ; let r : i8x4 = dsp_call ! (super :: __ssub8 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! usub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function usub8 in module {}", module_path!());
    };
}

mkfn!{
    usub8_introspect!();
    # [test] fn usub8 () { unsafe { let a = u8x4 :: new (1 , 2 , 3 , 4) ; let b = u8x4 :: new (5 , 4 , 3 , 2) ; let c = u8x4 :: new (252 , 254 , 0 , 2) ; let r : u8x4 = dsp_call ! (super :: __usub8 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! shsub16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function shsub16 in module {}", module_path!());
    };
}

mkfn!{
    shsub16_introspect!();
    # [test] fn shsub16 () { unsafe { let a = i16x2 :: new (1 , 2) ; let b = i16x2 :: new (5 , 4) ; let c = i16x2 :: new (- 2 , - 1) ; let r : i16x2 = dsp_call ! (super :: __shsub16 , a , b) ; assert_eq ! (r , c) ; } }
}

macro_rules! smuad_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smuad in module {}", module_path!());
    };
}

mkfn!{
    smuad_introspect!();
    # [test] fn smuad () { unsafe { let a = i16x2 :: new (1 , 2) ; let b = i16x2 :: new (5 , 4) ; let r = super :: __smuad (transmute (a) , transmute (b)) ; assert_eq ! (r , 13) ; } }
}

macro_rules! smuadx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smuadx in module {}", module_path!());
    };
}

mkfn!{
    smuadx_introspect!();
    # [test] fn smuadx () { unsafe { let a = i16x2 :: new (1 , 2) ; let b = i16x2 :: new (5 , 4) ; let r = super :: __smuadx (transmute (a) , transmute (b)) ; assert_eq ! (r , 14) ; } }
}

macro_rules! smusd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smusd in module {}", module_path!());
    };
}

mkfn!{
    smusd_introspect!();
    # [test] fn smusd () { unsafe { let a = i16x2 :: new (1 , 2) ; let b = i16x2 :: new (5 , 4) ; let r = super :: __smusd (transmute (a) , transmute (b)) ; assert_eq ! (r , - 3) ; } }
}

macro_rules! smusdx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smusdx in module {}", module_path!());
    };
}

mkfn!{
    smusdx_introspect!();
    # [test] fn smusdx () { unsafe { let a = i16x2 :: new (1 , 2) ; let b = i16x2 :: new (5 , 4) ; let r = super :: __smusdx (transmute (a) , transmute (b)) ; assert_eq ! (r , - 6) ; } }
}

macro_rules! usad8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function usad8 in module {}", module_path!());
    };
}

mkfn!{
    usad8_introspect!();
    # [test] fn usad8 () { unsafe { let a = i8x4 :: new (1 , 2 , 3 , 4) ; let b = i8x4 :: new (4 , 3 , 2 , 1) ; let r = super :: __usad8 (transmute (a) , transmute (b)) ; assert_eq ! (r , 8) ; } }
}

macro_rules! usad8a_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function usad8a in module {}", module_path!());
    };
}

mkfn!{
    usad8a_introspect!();
    # [test] fn usad8a () { unsafe { let a = i8x4 :: new (1 , 2 , 3 , 4) ; let b = i8x4 :: new (4 , 3 , 2 , 1) ; let c = 10 ; let r = super :: __usada8 (transmute (a) , transmute (b) , c) ; assert_eq ! (r , 8 + c) ; } }
} 
            }}