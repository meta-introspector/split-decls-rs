mkuse!{use crate :: arch :: asm ;}
mkmod!{zk, { 
                getname!(zk);
                getsrc!(zk);
                getpath!(zk);
                get_deps!(zk);
                get_crates!(zk);
                mkinclude!(zk);
                 
            }}
mkuse!{# [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub use zk :: * ;}

macro_rules! hlv_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hlv_wu in module {}", module_path!());
    };
}

mkfn!{
    hlv_wu_introspect!();
    # [doc = " Loads virtual machine memory by unsigned word integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " i.e., with the address translation and protection, and the endianness, that apply to memory"] # [doc = " accesses in either VS-mode or VU-mode."] # [doc = ""] # [doc = " This operation is not available under RV32 base instruction set."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HLV.WU`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hlv_wu (src : * const u32) -> u32 { let value : u32 ; asm ! (".insn i 0x73, 0x4, {}, {}, 0x681" , out (reg) value , in (reg) src , options (readonly , nostack)) ; value }
}

macro_rules! hlv_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hlv_d in module {}", module_path!());
    };
}

mkfn!{
    hlv_d_introspect!();
    # [doc = " Loads virtual machine memory by double integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " i.e., with the address translation and protection, and the endianness, that apply to memory"] # [doc = " accesses in either VS-mode or VU-mode."] # [doc = ""] # [doc = " This operation is not available under RV32 base instruction set."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HLV.D`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hlv_d (src : * const i64) -> i64 { let value : i64 ; asm ! (".insn i 0x73, 0x4, {}, {}, 0x6C0" , out (reg) value , in (reg) src , options (readonly , nostack)) ; value }
}

macro_rules! hsv_d_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hsv_d in module {}", module_path!());
    };
}

mkfn!{
    hsv_d_introspect!();
    # [doc = " Stores virtual machine memory by double integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " i.e., with the address translation and protection, and the endianness, that apply to memory"] # [doc = " accesses in either VS-mode or VU-mode."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HSV.D`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hsv_d (dst : * mut i64 , src : i64) { asm ! (".insn r 0x73, 0x4, 0x37, x0, {}, {}" , in (reg) dst , in (reg) src , options (nostack)) ; }
}