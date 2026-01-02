mkmod!{lasx, { 
                getname!(lasx);
                getsrc!(lasx);
                getpath!(lasx);
                get_deps!(lasx);
                get_crates!(lasx);
                mkinclude!(lasx);
                 
            }}
mkmod!{lsx, { 
                getname!(lsx);
                getsrc!(lsx);
                getpath!(lsx);
                get_deps!(lsx);
                get_crates!(lsx);
                mkinclude!(lsx);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub use self :: lasx :: * ;}
mkuse!{# [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub use self :: lsx :: * ;}
mkuse!{use crate :: arch :: asm ;}

macro_rules! rdtime_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rdtime_d in module {}", module_path!());
    };
}

mkfn!{
    rdtime_d_introspect!();
    # [doc = " Reads the 64-bit stable counter value and the counter ID"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn rdtime_d () -> (i64 , isize) { let (val , tid) : (i64 , isize) ; unsafe { asm ! ("rdtime.d {}, {}" , out (reg) val , out (reg) tid , options (readonly , nostack)) } ; (val , tid) }
}
mkitem!{# [allow (improper_ctypes)] unsafe extern "unadjusted" { # [link_name = "llvm.loongarch.crc.w.d.w"] fn __crc_w_d_w (a : i64 , b : i32) -> i32 ; # [link_name = "llvm.loongarch.crcc.w.d.w"] fn __crcc_w_d_w (a : i64 , b : i32) -> i32 ; # [link_name = "llvm.loongarch.cacop.d"] fn __cacop (a : i64 , b : i64 , c : i64) ; # [link_name = "llvm.loongarch.csrrd.d"] fn __csrrd (a : i32) -> i64 ; # [link_name = "llvm.loongarch.csrwr.d"] fn __csrwr (a : i64 , b : i32) -> i64 ; # [link_name = "llvm.loongarch.csrxchg.d"] fn __csrxchg (a : i64 , b : i64 , c : i32) -> i64 ; # [link_name = "llvm.loongarch.iocsrrd.d"] fn __iocsrrd_d (a : i32) -> i64 ; # [link_name = "llvm.loongarch.iocsrwr.d"] fn __iocsrwr_d (a : i64 , b : i32) ; # [link_name = "llvm.loongarch.asrtle.d"] fn __asrtle (a : i64 , b : i64) ; # [link_name = "llvm.loongarch.asrtgt.d"] fn __asrtgt (a : i64 , b : i64) ; # [link_name = "llvm.loongarch.lddir.d"] fn __lddir (a : i64 , b : i64) -> i64 ; # [link_name = "llvm.loongarch.ldpte.d"] fn __ldpte (a : i64 , b : i64) ; }}

macro_rules! crc_w_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crc_w_d_w in module {}", module_path!());
    };
}

mkfn!{
    crc_w_d_w_introspect!();
    # [doc = " Calculate the CRC value using the IEEE 802.3 polynomial (0xEDB88320)"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn crc_w_d_w (a : i64 , b : i32) -> i32 { unsafe { __crc_w_d_w (a , b) } }
}

macro_rules! crcc_w_d_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crcc_w_d_w in module {}", module_path!());
    };
}

mkfn!{
    crcc_w_d_w_introspect!();
    # [doc = " Calculate the CRC value using the Castagnoli polynomial (0x82F63B78)"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub fn crcc_w_d_w (a : i64 , b : i32) -> i32 { unsafe { __crcc_w_d_w (a , b) } }
}

macro_rules! cacop_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cacop in module {}", module_path!());
    };
}

mkfn!{
    cacop_introspect!();
    # [doc = " Generates the cache operation instruction"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn cacop < const IMM12 : i64 > (a : i64 , b : i64) { static_assert_simm_bits ! (IMM12 , 12) ; __cacop (a , b , IMM12) ; }
}

macro_rules! csrrd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function csrrd in module {}", module_path!());
    };
}

mkfn!{
    csrrd_introspect!();
    # [doc = " Reads the CSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn csrrd < const IMM14 : i32 > () -> i64 { static_assert_uimm_bits ! (IMM14 , 14) ; __csrrd (IMM14) }
}

macro_rules! csrwr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function csrwr in module {}", module_path!());
    };
}

mkfn!{
    csrwr_introspect!();
    # [doc = " Writes the CSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn csrwr < const IMM14 : i32 > (a : i64) -> i64 { static_assert_uimm_bits ! (IMM14 , 14) ; __csrwr (a , IMM14) }
}

macro_rules! csrxchg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function csrxchg in module {}", module_path!());
    };
}

mkfn!{
    csrxchg_introspect!();
    # [doc = " Exchanges the CSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn csrxchg < const IMM14 : i32 > (a : i64 , b : i64) -> i64 { static_assert_uimm_bits ! (IMM14 , 14) ; __csrxchg (a , b , IMM14) }
}

macro_rules! iocsrrd_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iocsrrd_d in module {}", module_path!());
    };
}

mkfn!{
    iocsrrd_d_introspect!();
    # [doc = " Reads the 64-bit IO-CSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn iocsrrd_d (a : i32) -> i64 { __iocsrrd_d (a) }
}

macro_rules! iocsrwr_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iocsrwr_d in module {}", module_path!());
    };
}

mkfn!{
    iocsrwr_d_introspect!();
    # [doc = " Writes the 64-bit IO-CSR"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn iocsrwr_d (a : i64 , b : i32) { __iocsrwr_d (a , b) }
}

macro_rules! asrtle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function asrtle in module {}", module_path!());
    };
}

mkfn!{
    asrtle_introspect!();
    # [doc = " Generates the less-than-or-equal asseration instruction"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn asrtle (a : i64 , b : i64) { __asrtle (a , b) ; }
}

macro_rules! asrtgt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function asrtgt in module {}", module_path!());
    };
}

mkfn!{
    asrtgt_introspect!();
    # [doc = " Generates the greater-than asseration instruction"] # [inline] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn asrtgt (a : i64 , b : i64) { __asrtgt (a , b) ; }
}

macro_rules! lddir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lddir in module {}", module_path!());
    };
}

mkfn!{
    lddir_introspect!();
    # [doc = " Loads the page table directory entry"] # [inline] # [rustc_legacy_const_generics (1)] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn lddir < const B : i64 > (a : i64) -> i64 { __lddir (a , B) }
}

macro_rules! ldpte_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ldpte in module {}", module_path!());
    };
}

mkfn!{
    ldpte_introspect!();
    # [doc = " Loads the page table entry"] # [inline] # [rustc_legacy_const_generics (1)] # [unstable (feature = "stdarch_loongarch" , issue = "117427")] pub unsafe fn ldpte < const B : i64 > (a : i64) { __ldpte (a , B) }
}