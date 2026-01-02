mkuse!{use crate :: arch :: asm ;}

macro_rules! rdtimel_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rdtimel_w in module {}", module_path!());
    };
}

mkfn!{
    rdtimel_w_introspect!();
    # [doc = " Reads the lower 32-bit stable counter value and the counter ID"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn rdtimel_w () -> (i32 , isize) { let (val , tid) : (i32 , isize) ; unsafe { asm ! ("rdtimel.w {}, {}" , out (reg) val , out (reg) tid , options (readonly , nostack)) } ; (val , tid) }
}

macro_rules! rdtimeh_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rdtimeh_w in module {}", module_path!());
    };
}

mkfn!{
    rdtimeh_w_introspect!();
    # [doc = " Reads the upper 32-bit stable counter value and the counter ID"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn rdtimeh_w () -> (i32 , isize) { let (val , tid) : (i32 , isize) ; unsafe { asm ! ("rdtimeh.w {}, {}" , out (reg) val , out (reg) tid , options (readonly , nostack)) } ; (val , tid) }
}
mkitem!{# [allow (improper_ctypes)] unsafe extern "unadjusted" { # [link_name = "llvm.loongarch.crc.w.b.w"] fn __crc_w_b_w (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.loongarch.crc.w.h.w"] fn __crc_w_h_w (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.loongarch.crc.w.w.w"] fn __crc_w_w_w (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.loongarch.crcc.w.b.w"] fn __crcc_w_b_w (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.loongarch.crcc.w.h.w"] fn __crcc_w_h_w (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.loongarch.crcc.w.w.w"] fn __crcc_w_w_w (a : i32 , b : i32) -> i32 ; # [link_name = "llvm.loongarch.dbar"] fn __dbar (a : i32) ; # [link_name = "llvm.loongarch.ibar"] fn __ibar (a : i32) ; # [link_name = "llvm.loongarch.movgr2fcsr"] fn __movgr2fcsr (a : i32 , b : i32) ; # [link_name = "llvm.loongarch.movfcsr2gr"] fn __movfcsr2gr (a : i32) -> i32 ; # [link_name = "llvm.loongarch.iocsrrd.b"] fn __iocsrrd_b (a : i32) -> i32 ; # [link_name = "llvm.loongarch.iocsrrd.h"] fn __iocsrrd_h (a : i32) -> i32 ; # [link_name = "llvm.loongarch.iocsrrd.w"] fn __iocsrrd_w (a : i32) -> i32 ; # [link_name = "llvm.loongarch.iocsrwr.b"] fn __iocsrwr_b (a : i32 , b : i32) ; # [link_name = "llvm.loongarch.iocsrwr.h"] fn __iocsrwr_h (a : i32 , b : i32) ; # [link_name = "llvm.loongarch.iocsrwr.w"] fn __iocsrwr_w (a : i32 , b : i32) ; # [link_name = "llvm.loongarch.break"] fn __break (a : i32) ; # [link_name = "llvm.loongarch.cpucfg"] fn __cpucfg (a : i32) -> i32 ; # [link_name = "llvm.loongarch.syscall"] fn __syscall (a : i32) ; # [link_name = "llvm.loongarch.frecipe.s"] fn __frecipe_s (a : f32) -> f32 ; # [link_name = "llvm.loongarch.frecipe.d"] fn __frecipe_d (a : f64) -> f64 ; # [link_name = "llvm.loongarch.frsqrte.s"] fn __frsqrte_s (a : f32) -> f32 ; # [link_name = "llvm.loongarch.frsqrte.d"] fn __frsqrte_d (a : f64) -> f64 ; }}

macro_rules! crc_w_b_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crc_w_b_w in module {}", module_path!());
    };
}

mkfn!{
    crc_w_b_w_introspect!();
    # [doc = " Calculate the CRC value using the IEEE 802.3 polynomial (0xEDB88320)"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn crc_w_b_w (a : i32 , b : i32) -> i32 { unsafe { __crc_w_b_w (a , b) } }
}

macro_rules! crc_w_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crc_w_h_w in module {}", module_path!());
    };
}

mkfn!{
    crc_w_h_w_introspect!();
    # [doc = " Calculate the CRC value using the IEEE 802.3 polynomial (0xEDB88320)"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn crc_w_h_w (a : i32 , b : i32) -> i32 { unsafe { __crc_w_h_w (a , b) } }
}

macro_rules! crc_w_w_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crc_w_w_w in module {}", module_path!());
    };
}

mkfn!{
    crc_w_w_w_introspect!();
    # [doc = " Calculate the CRC value using the IEEE 802.3 polynomial (0xEDB88320)"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn crc_w_w_w (a : i32 , b : i32) -> i32 { unsafe { __crc_w_w_w (a , b) } }
}

macro_rules! crcc_w_b_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crcc_w_b_w in module {}", module_path!());
    };
}

mkfn!{
    crcc_w_b_w_introspect!();
    # [doc = " Calculate the CRC value using the Castagnoli polynomial (0x82F63B78)"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn crcc_w_b_w (a : i32 , b : i32) -> i32 { unsafe { __crcc_w_b_w (a , b) } }
}

macro_rules! crcc_w_h_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crcc_w_h_w in module {}", module_path!());
    };
}

mkfn!{
    crcc_w_h_w_introspect!();
    # [doc = " Calculate the CRC value using the Castagnoli polynomial (0x82F63B78)"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn crcc_w_h_w (a : i32 , b : i32) -> i32 { unsafe { __crcc_w_h_w (a , b) } }
}

macro_rules! crcc_w_w_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crcc_w_w_w in module {}", module_path!());
    };
}

mkfn!{
    crcc_w_w_w_introspect!();
    # [doc = " Calculate the CRC value using the Castagnoli polynomial (0x82F63B78)"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn crcc_w_w_w (a : i32 , b : i32) -> i32 { unsafe { __crcc_w_w_w (a , b) } }
}

macro_rules! dbar_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dbar in module {}", module_path!());
    };
}

mkfn!{
    dbar_introspect!();
    # [doc = " Generates the memory barrier instruction"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn dbar < const IMM15 : i32 > () { static_assert_uimm_bits ! (IMM15 , 15) ; unsafe { __dbar (IMM15) } ; }
}

macro_rules! ibar_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ibar in module {}", module_path!());
    };
}

mkfn!{
    ibar_introspect!();
    # [doc = " Generates the instruction-fetch barrier instruction"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn ibar < const IMM15 : i32 > () { static_assert_uimm_bits ! (IMM15 , 15) ; unsafe { __ibar (IMM15) } ; }
}

macro_rules! movgr2fcsr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function movgr2fcsr in module {}", module_path!());
    };
}

mkfn!{
    movgr2fcsr_introspect!();
    # [doc = " Moves data from a GPR to the FCSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn movgr2fcsr < const IMM5 : i32 > (a : i32) { static_assert_uimm_bits ! (IMM5 , 5) ; __movgr2fcsr (IMM5 , a) ; }
}

macro_rules! movfcsr2gr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function movfcsr2gr in module {}", module_path!());
    };
}

mkfn!{
    movfcsr2gr_introspect!();
    # [doc = " Moves data from a FCSR to the GPR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn movfcsr2gr < const IMM5 : i32 > () -> i32 { static_assert_uimm_bits ! (IMM5 , 5) ; unsafe { __movfcsr2gr (IMM5) } }
}

macro_rules! iocsrrd_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iocsrrd_b in module {}", module_path!());
    };
}

mkfn!{
    iocsrrd_b_introspect!();
    # [doc = " Reads the 8-bit IO-CSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn iocsrrd_b (a : i32) -> i32 { __iocsrrd_b (a) }
}

macro_rules! iocsrrd_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iocsrrd_h in module {}", module_path!());
    };
}

mkfn!{
    iocsrrd_h_introspect!();
    # [doc = " Reads the 16-bit IO-CSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn iocsrrd_h (a : i32) -> i32 { __iocsrrd_h (a) }
}

macro_rules! iocsrrd_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iocsrrd_w in module {}", module_path!());
    };
}

mkfn!{
    iocsrrd_w_introspect!();
    # [doc = " Reads the 32-bit IO-CSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn iocsrrd_w (a : i32) -> i32 { __iocsrrd_w (a) }
}

macro_rules! iocsrwr_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iocsrwr_b in module {}", module_path!());
    };
}

mkfn!{
    iocsrwr_b_introspect!();
    # [doc = " Writes the 8-bit IO-CSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn iocsrwr_b (a : i32 , b : i32) { __iocsrwr_b (a , b) }
}

macro_rules! iocsrwr_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iocsrwr_h in module {}", module_path!());
    };
}

mkfn!{
    iocsrwr_h_introspect!();
    # [doc = " Writes the 16-bit IO-CSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn iocsrwr_h (a : i32 , b : i32) { __iocsrwr_h (a , b) }
}

macro_rules! iocsrwr_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iocsrwr_w in module {}", module_path!());
    };
}

mkfn!{
    iocsrwr_w_introspect!();
    # [doc = " Writes the 32-bit IO-CSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn iocsrwr_w (a : i32 , b : i32) { __iocsrwr_w (a , b) }
}

macro_rules! brk_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function brk in module {}", module_path!());
    };
}

mkfn!{
    brk_introspect!();
    # [doc = " Generates the breakpoint instruction"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn brk < const IMM15 : i32 > () { static_assert_uimm_bits ! (IMM15 , 15) ; __break (IMM15) ; }
}

macro_rules! cpucfg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cpucfg in module {}", module_path!());
    };
}

mkfn!{
    cpucfg_introspect!();
    # [doc = " Reads the CPU configuration register"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn cpucfg (a : i32) -> i32 { unsafe { __cpucfg (a) } }
}

macro_rules! syscall_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function syscall in module {}", module_path!());
    };
}

mkfn!{
    syscall_introspect!();
    # [doc = " Generates the syscall instruction"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn syscall < const IMM15 : i32 > () { static_assert_uimm_bits ! (IMM15 , 15) ; __syscall (IMM15) ; }
}

macro_rules! frecipe_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function frecipe_s in module {}", module_path!());
    };
}

mkfn!{
    frecipe_s_introspect!();
    # [doc = " Calculate the approximate single-precision result of 1.0 divided"] # [inline] # [target_feature (enable = "frecipe")] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn frecipe_s (a : f32) -> f32 { unsafe { __frecipe_s (a) } }
}

macro_rules! frecipe_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function frecipe_d in module {}", module_path!());
    };
}

mkfn!{
    frecipe_d_introspect!();
    # [doc = " Calculate the approximate double-precision result of 1.0 divided"] # [inline] # [target_feature (enable = "frecipe")] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn frecipe_d (a : f64) -> f64 { unsafe { __frecipe_d (a) } }
}

macro_rules! frsqrte_s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function frsqrte_s in module {}", module_path!());
    };
}

mkfn!{
    frsqrte_s_introspect!();
    # [doc = " Calculate the approximate single-precision result of dividing 1.0 by the square root"] # [inline] # [target_feature (enable = "frecipe")] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn frsqrte_s (a : f32) -> f32 { unsafe { __frsqrte_s (a) } }
}

macro_rules! frsqrte_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function frsqrte_d in module {}", module_path!());
    };
}

mkfn!{
    frsqrte_d_introspect!();
    # [doc = " Calculate the approximate double-precision result of dividing 1.0 by the square root"] # [inline] # [target_feature (enable = "frecipe")] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn frsqrte_d (a : f64) -> f64 { unsafe { __frsqrte_d (a) } }
}