mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{# [cfg (target_arch = "riscv32")] unsafe extern "unadjusted" { # [link_name = "llvm.riscv.orc.b.i32"] fn _orc_b_32 (rs : i32) -> i32 ; # [link_name = "llvm.riscv.clmul.i32"] fn _clmul_32 (rs1 : i32 , rs2 : i32) -> i32 ; # [link_name = "llvm.riscv.clmulh.i32"] fn _clmulh_32 (rs1 : i32 , rs2 : i32) -> i32 ; # [link_name = "llvm.riscv.clmulr.i32"] fn _clmulr_32 (rs1 : i32 , rs2 : i32) -> i32 ; }}
mkitem!{# [cfg (target_arch = "riscv64")] unsafe extern "unadjusted" { # [link_name = "llvm.riscv.orc.b.i64"] fn _orc_b_64 (rs1 : i64) -> i64 ; # [link_name = "llvm.riscv.clmul.i64"] fn _clmul_64 (rs1 : i64 , rs2 : i64) -> i64 ; # [link_name = "llvm.riscv.clmulh.i64"] fn _clmulh_64 (rs1 : i64 , rs2 : i64) -> i64 ; # [link_name = "llvm.riscv.clmulr.i64"] fn _clmulr_64 (rs1 : i64 , rs2 : i64) -> i64 ; }}

macro_rules! orc_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function orc_b in module {}", module_path!());
    };
}

mkfn!{
    orc_b_introspect!();
    # [doc = " Bitwise OR-Combine, byte granule"] # [doc = ""] # [doc = " Combines the bits within every byte through a reciprocal bitwise logical OR. This sets the bits of each byte in"] # [doc = " the result rd to all zeros if no bit within the respective byte of rs is set, or to all ones if any bit within the"] # [doc = " respective byte of rs is set."] # [doc = ""] # [doc = " Source: RISC-V Bit-Manipulation ISA-extensions"] # [doc = ""] # [doc = " Version: v1.0.0"] # [doc = ""] # [doc = " Section: 2.24"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zbb")] # [cfg_attr (test , assert_instr (orc . b))] # [inline] pub fn orc_b (rs : usize) -> usize { # [cfg (target_arch = "riscv32")] unsafe { _orc_b_32 (rs as i32) as usize } # [cfg (target_arch = "riscv64")] unsafe { _orc_b_64 (rs as i64) as usize } }
}

macro_rules! clmul_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clmul in module {}", module_path!());
    };
}

mkfn!{
    clmul_introspect!();
    # [doc = " Carry-less multiply (low-part)"] # [doc = ""] # [doc = " clmul produces the lower half of the 2·XLEN carry-less product."] # [doc = ""] # [doc = " Source: RISC-V Bit-Manipulation ISA-extensions"] # [doc = ""] # [doc = " Version: v1.0.0"] # [doc = ""] # [doc = " Section: 2.11"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zbc")] # [cfg_attr (test , assert_instr (clmul))] # [inline] pub fn clmul (rs1 : usize , rs2 : usize) -> usize { # [cfg (target_arch = "riscv32")] unsafe { _clmul_32 (rs1 as i32 , rs2 as i32) as usize } # [cfg (target_arch = "riscv64")] unsafe { _clmul_64 (rs1 as i64 , rs2 as i64) as usize } }
}

macro_rules! clmulh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clmulh in module {}", module_path!());
    };
}

mkfn!{
    clmulh_introspect!();
    # [doc = " Carry-less multiply (high-part)"] # [doc = ""] # [doc = " clmulh produces the upper half of the 2·XLEN carry-less product."] # [doc = ""] # [doc = " Source: RISC-V Bit-Manipulation ISA-extensions"] # [doc = ""] # [doc = " Version: v1.0.0"] # [doc = ""] # [doc = " Section: 2.12"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zbc")] # [cfg_attr (test , assert_instr (clmulh))] # [inline] pub fn clmulh (rs1 : usize , rs2 : usize) -> usize { # [cfg (target_arch = "riscv32")] unsafe { _clmulh_32 (rs1 as i32 , rs2 as i32) as usize } # [cfg (target_arch = "riscv64")] unsafe { _clmulh_64 (rs1 as i64 , rs2 as i64) as usize } }
}

macro_rules! clmulr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clmulr in module {}", module_path!());
    };
}

mkfn!{
    clmulr_introspect!();
    # [doc = " Carry-less multiply (reversed)"] # [doc = ""] # [doc = " clmulr produces bits 2·XLEN−2:XLEN-1 of the 2·XLEN carry-less product."] # [doc = ""] # [doc = " Source: RISC-V Bit-Manipulation ISA-extensions"] # [doc = ""] # [doc = " Version: v1.0.0"] # [doc = ""] # [doc = " Section: 2.13"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zbc")] # [cfg_attr (test , assert_instr (clmulr))] # [inline] pub fn clmulr (rs1 : usize , rs2 : usize) -> usize { # [cfg (target_arch = "riscv32")] unsafe { _clmulr_32 (rs1 as i32 , rs2 as i32) as usize } # [cfg (target_arch = "riscv64")] unsafe { _clmulr_64 (rs1 as i64 , rs2 as i64) as usize } }
}