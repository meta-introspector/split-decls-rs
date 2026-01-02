mkmod!{p, { 
                getname!(p);
                getsrc!(p);
                getpath!(p);
                get_deps!(p);
                get_crates!(p);
                mkinclude!(p);
                 
            }}
mkmod!{zb, { 
                getname!(zb);
                getsrc!(zb);
                getpath!(zb);
                get_deps!(zb);
                get_crates!(zb);
                mkinclude!(zb);
                 
            }}
mkmod!{zk, { 
                getname!(zk);
                getsrc!(zk);
                getpath!(zk);
                get_deps!(zk);
                get_crates!(zk);
                mkinclude!(zk);
                 
            }}
mkuse!{# [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub use p :: * ;}
mkuse!{# [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub use zb :: * ;}
mkuse!{# [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub use zk :: * ;}
mkuse!{use crate :: arch :: asm ;}

macro_rules! pause_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pause in module {}", module_path!());
    };
}

mkfn!{
    pause_introspect!();
    # [doc = " Generates the `PAUSE` instruction"] # [doc = ""] # [doc = " The PAUSE instruction is a HINT that indicates the current hart's rate of instruction retirement"] # [doc = " should be temporarily reduced or paused. The duration of its effect must be bounded and may be zero."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn pause () { unsafe { asm ! (".insn i 0x0F, 0, x0, x0, 0x010" , options (nomem , nostack)) } }
}

macro_rules! nop_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nop in module {}", module_path!());
    };
}

mkfn!{
    nop_introspect!();
    # [doc = " Generates the `NOP` instruction"] # [doc = ""] # [doc = " The NOP instruction does not change any architecturally visible state, except for"] # [doc = " advancing the `pc` and incrementing any applicable performance counters."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn nop () { unsafe { asm ! ("nop" , options (nomem , nostack)) } }
}

macro_rules! wfi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wfi in module {}", module_path!());
    };
}

mkfn!{
    wfi_introspect!();
    # [doc = " Generates the `WFI` instruction"] # [doc = ""] # [doc = " The WFI instruction provides a hint to the implementation that the current hart can be stalled"] # [doc = " until an interrupt might need servicing. This instruction is a hint,"] # [doc = " and a legal implementation is to simply implement WFI as a NOP."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn wfi () { asm ! ("wfi" , options (nomem , nostack)) }
}

macro_rules! fence_i_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fence_i in module {}", module_path!());
    };
}

mkfn!{
    fence_i_introspect!();
    # [doc = " Generates the `FENCE.I` instruction"] # [doc = ""] # [doc = " A FENCE.I instruction ensures that a subsequent instruction fetch on a RISC-V hart will see"] # [doc = " any previous data stores already visible to the same RISC-V hart."] # [doc = ""] # [doc = " FENCE.I does not ensure that other RISC-V harts' instruction fetches will observe the"] # [doc = " local hart's stores in a multiprocessor system."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn fence_i () { asm ! ("fence.i" , options (nostack)) }
}

macro_rules! sfence_vma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sfence_vma in module {}", module_path!());
    };
}

mkfn!{
    sfence_vma_introspect!();
    # [doc = " Supervisor memory management fence for given virtual address and address space"] # [doc = ""] # [doc = " The fence orders only reads and writes made to leaf page table entries corresponding to"] # [doc = " the virtual address in parameter `vaddr`, for the address space identified by integer parameter"] # [doc = " `asid`. Accesses to global mappings are not ordered. The fence also invalidates all"] # [doc = " address-translation cache entries that contain leaf page table entries corresponding to the"] # [doc = " virtual address in parameter `vaddr` and that match the address space identified by integer"] # [doc = " parameter `asid`, except for entries containing global mappings."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn sfence_vma (vaddr : usize , asid : usize) { asm ! ("sfence.vma {}, {}" , in (reg) vaddr , in (reg) asid , options (nostack)) }
}

macro_rules! sfence_vma_vaddr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sfence_vma_vaddr in module {}", module_path!());
    };
}

mkfn!{
    sfence_vma_vaddr_introspect!();
    # [doc = " Supervisor memory management fence for given virtual address"] # [doc = ""] # [doc = " The fence orders only reads and writes made to leaf page table entries corresponding to"] # [doc = " the virtual address in parameter `vaddr`, for all address spaces."] # [doc = " The fence also invalidates all address-translation cache entries that contain leaf page"] # [doc = " table entries corresponding to the virtual address in parameter `vaddr`, for all address spaces."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn sfence_vma_vaddr (vaddr : usize) { asm ! ("sfence.vma {}, x0" , in (reg) vaddr , options (nostack)) }
}

macro_rules! sfence_vma_asid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sfence_vma_asid in module {}", module_path!());
    };
}

mkfn!{
    sfence_vma_asid_introspect!();
    # [doc = " Supervisor memory management fence for given address space"] # [doc = ""] # [doc = " The fence orders all reads and writes made to any level of the page tables,"] # [doc = " but only for the address space identified by integer parameter `asid`."] # [doc = ""] # [doc = " Accesses to global mappings are not ordered. The fence also invalidates all"] # [doc = " address-translation cache entries matching the address space identified by integer"] # [doc = " parameter `asid`, except for entries containing global mappings."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn sfence_vma_asid (asid : usize) { asm ! ("sfence.vma x0, {}" , in (reg) asid , options (nostack)) }
}

macro_rules! sfence_vma_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sfence_vma_all in module {}", module_path!());
    };
}

mkfn!{
    sfence_vma_all_introspect!();
    # [doc = " Supervisor memory management fence for all address spaces and virtual addresses"] # [doc = ""] # [doc = " The fence orders all reads and writes made to any level of the page"] # [doc = " tables, for all address spaces. The fence also invalidates all address-translation cache entries,"] # [doc = " for all address spaces."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn sfence_vma_all () { asm ! ("sfence.vma" , options (nostack)) }
}

macro_rules! sinval_vma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sinval_vma in module {}", module_path!());
    };
}

mkfn!{
    sinval_vma_introspect!();
    # [doc = " Invalidate supervisor translation cache for given virtual address and address space"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `SFENCE.VMA` instruction with the same values of `vaddr` and `asid` would invalidate."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn sinval_vma (vaddr : usize , asid : usize) { asm ! (".insn r 0x73, 0, 0x0B, x0, {}, {}" , in (reg) vaddr , in (reg) asid , options (nostack)) }
}

macro_rules! sinval_vma_vaddr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sinval_vma_vaddr in module {}", module_path!());
    };
}

mkfn!{
    sinval_vma_vaddr_introspect!();
    # [doc = " Invalidate supervisor translation cache for given virtual address"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `SFENCE.VMA` instruction with the same values of `vaddr` and `asid` would invalidate."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn sinval_vma_vaddr (vaddr : usize) { asm ! (".insn r 0x73, 0, 0x0B, x0, {}, x0" , in (reg) vaddr , options (nostack)) }
}

macro_rules! sinval_vma_asid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sinval_vma_asid in module {}", module_path!());
    };
}

mkfn!{
    sinval_vma_asid_introspect!();
    # [doc = " Invalidate supervisor translation cache for given address space"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `SFENCE.VMA` instruction with the same values of `vaddr` and `asid` would invalidate."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn sinval_vma_asid (asid : usize) { asm ! (".insn r 0x73, 0, 0x0B, x0, x0, {}" , in (reg) asid , options (nostack)) }
}

macro_rules! sinval_vma_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sinval_vma_all in module {}", module_path!());
    };
}

mkfn!{
    sinval_vma_all_introspect!();
    # [doc = " Invalidate supervisor translation cache for all address spaces and virtual addresses"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `SFENCE.VMA` instruction with the same values of `vaddr` and `asid` would invalidate."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn sinval_vma_all () { asm ! (".insn r 0x73, 0, 0x0B, x0, x0, x0" , options (nostack)) }
}

macro_rules! sfence_w_inval_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sfence_w_inval in module {}", module_path!());
    };
}

mkfn!{
    sfence_w_inval_introspect!();
    # [doc = " Generates the `SFENCE.W.INVAL` instruction"] # [doc = ""] # [doc = " This instruction guarantees that any previous stores already visible to the current RISC-V hart"] # [doc = " are ordered before subsequent `SINVAL.VMA` instructions executed by the same hart."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn sfence_w_inval () { asm ! (".insn i 0x73, 0, x0, x0, 0x180" , options (nostack)) }
}

macro_rules! sfence_inval_ir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sfence_inval_ir in module {}", module_path!());
    };
}

mkfn!{
    sfence_inval_ir_introspect!();
    # [doc = " Generates the `SFENCE.INVAL.IR` instruction"] # [doc = ""] # [doc = " This instruction guarantees that any previous SINVAL.VMA instructions executed by the current hart"] # [doc = " are ordered before subsequent implicit references by that hart to the memory-management data structures."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn sfence_inval_ir () { asm ! (".insn i 0x73, 0, x0, x0, 0x181" , options (nostack)) }
}

macro_rules! hlv_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hlv_b in module {}", module_path!());
    };
}

mkfn!{
    hlv_b_introspect!();
    # [doc = " Loads virtual machine memory by signed byte integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " i.e., with the address translation and protection, and the endianness, that apply to memory"] # [doc = " accesses in either VS-mode or VU-mode."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HLV.B`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hlv_b (src : * const i8) -> i8 { let value : i8 ; asm ! (".insn i 0x73, 0x4, {}, {}, 0x600" , out (reg) value , in (reg) src , options (readonly , nostack)) ; value }
}

macro_rules! hlv_bu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hlv_bu in module {}", module_path!());
    };
}

mkfn!{
    hlv_bu_introspect!();
    # [doc = " Loads virtual machine memory by unsigned byte integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " i.e., with the address translation and protection, and the endianness, that apply to memory"] # [doc = " accesses in either VS-mode or VU-mode."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HLV.BU`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hlv_bu (src : * const u8) -> u8 { let value : u8 ; asm ! (".insn i 0x73, 0x4, {}, {}, 0x601" , out (reg) value , in (reg) src , options (readonly , nostack)) ; value }
}

macro_rules! hlv_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hlv_h in module {}", module_path!());
    };
}

mkfn!{
    hlv_h_introspect!();
    # [doc = " Loads virtual machine memory by signed half integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " i.e., with the address translation and protection, and the endianness, that apply to memory"] # [doc = " accesses in either VS-mode or VU-mode."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HLV.H`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hlv_h (src : * const i16) -> i16 { let value : i16 ; asm ! (".insn i 0x73, 0x4, {}, {}, 0x640" , out (reg) value , in (reg) src , options (readonly , nostack)) ; value }
}

macro_rules! hlv_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hlv_hu in module {}", module_path!());
    };
}

mkfn!{
    hlv_hu_introspect!();
    # [doc = " Loads virtual machine memory by unsigned half integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " i.e., with the address translation and protection, and the endianness, that apply to memory"] # [doc = " accesses in either VS-mode or VU-mode."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HLV.HU`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hlv_hu (src : * const u16) -> u16 { let value : u16 ; asm ! (".insn i 0x73, 0x4, {}, {}, 0x641" , out (reg) value , in (reg) src , options (readonly , nostack)) ; value }
}

macro_rules! hlvx_hu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hlvx_hu in module {}", module_path!());
    };
}

mkfn!{
    hlvx_hu_introspect!();
    # [doc = " Accesses virtual machine instruction by unsigned half integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " the memory being read must be executable in both stages of address translation,"] # [doc = " but read permission is not required."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HLVX.HU`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hlvx_hu (src : * const u16) -> u16 { let insn : u16 ; asm ! (".insn i 0x73, 0x4, {}, {}, 0x643" , out (reg) insn , in (reg) src , options (readonly , nostack)) ; insn }
}

macro_rules! hlv_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hlv_w in module {}", module_path!());
    };
}

mkfn!{
    hlv_w_introspect!();
    # [doc = " Loads virtual machine memory by signed word integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " i.e., with the address translation and protection, and the endianness, that apply to memory"] # [doc = " accesses in either VS-mode or VU-mode."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HLV.W`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hlv_w (src : * const i32) -> i32 { let value : i32 ; asm ! (".insn i 0x73, 0x4, {}, {}, 0x680" , out (reg) value , in (reg) src , options (readonly , nostack)) ; value }
}

macro_rules! hlvx_wu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hlvx_wu in module {}", module_path!());
    };
}

mkfn!{
    hlvx_wu_introspect!();
    # [doc = " Accesses virtual machine instruction by unsigned word integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " the memory being read must be executable in both stages of address translation,"] # [doc = " but read permission is not required."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HLVX.WU`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hlvx_wu (src : * const u32) -> u32 { let insn : u32 ; asm ! (".insn i 0x73, 0x4, {}, {}, 0x683" , out (reg) insn , in (reg) src , options (readonly , nostack)) ; insn }
}

macro_rules! hsv_b_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hsv_b in module {}", module_path!());
    };
}

mkfn!{
    hsv_b_introspect!();
    # [doc = " Stores virtual machine memory by byte integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " i.e., with the address translation and protection, and the endianness, that apply to memory"] # [doc = " accesses in either VS-mode or VU-mode."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HSV.B`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hsv_b (dst : * mut i8 , src : i8) { asm ! (".insn r 0x73, 0x4, 0x31, x0, {}, {}" , in (reg) dst , in (reg) src , options (nostack)) ; }
}

macro_rules! hsv_h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hsv_h in module {}", module_path!());
    };
}

mkfn!{
    hsv_h_introspect!();
    # [doc = " Stores virtual machine memory by half integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " i.e., with the address translation and protection, and the endianness, that apply to memory"] # [doc = " accesses in either VS-mode or VU-mode."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HSV.H`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hsv_h (dst : * mut i16 , src : i16) { asm ! (".insn r 0x73, 0x4, 0x33, x0, {}, {}" , in (reg) dst , in (reg) src , options (nostack)) ; }
}

macro_rules! hsv_w_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hsv_w in module {}", module_path!());
    };
}

mkfn!{
    hsv_w_introspect!();
    # [doc = " Stores virtual machine memory by word integer"] # [doc = ""] # [doc = " This instruction performs an explicit memory access as though `V=1`;"] # [doc = " i.e., with the address translation and protection, and the endianness, that apply to memory"] # [doc = " accesses in either VS-mode or VU-mode."] # [doc = ""] # [doc = " This function is unsafe for it accesses the virtual supervisor or user via a `HSV.W`"] # [doc = " instruction which is effectively a dereference to any memory address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hsv_w (dst : * mut i32 , src : i32) { asm ! (".insn r 0x73, 0x4, 0x35, x0, {}, {}" , in (reg) dst , in (reg) src , options (nostack)) ; }
}

macro_rules! hfence_vvma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hfence_vvma in module {}", module_path!());
    };
}

mkfn!{
    hfence_vvma_introspect!();
    # [doc = " Hypervisor memory management fence for given guest virtual address and guest address space"] # [doc = ""] # [doc = " Guarantees that any previous stores already visible to the current hart are ordered before all"] # [doc = " implicit reads by that hart done for VS-stage address translation for instructions that:"] # [doc = " - are subsequent to the `HFENCE.VVMA`, and"] # [doc = " - execute when `hgatp.VMID` has the same setting as it did when `HFENCE.VVMA` executed."] # [doc = ""] # [doc = " This fence specifies a single guest virtual address, and a single guest address-space identifier."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hfence_vvma (vaddr : usize , asid : usize) { asm ! (".insn r 0x73, 0, 0x11, x0, {}, {}" , in (reg) vaddr , in (reg) asid , options (nostack)) }
}

macro_rules! hfence_vvma_vaddr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hfence_vvma_vaddr in module {}", module_path!());
    };
}

mkfn!{
    hfence_vvma_vaddr_introspect!();
    # [doc = " Hypervisor memory management fence for given guest virtual address"] # [doc = ""] # [doc = " Guarantees that any previous stores already visible to the current hart are ordered before all"] # [doc = " implicit reads by that hart done for VS-stage address translation for instructions that:"] # [doc = " - are subsequent to the `HFENCE.VVMA`, and"] # [doc = " - execute when `hgatp.VMID` has the same setting as it did when `HFENCE.VVMA` executed."] # [doc = ""] # [doc = " This fence specifies a single guest virtual address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hfence_vvma_vaddr (vaddr : usize) { asm ! (".insn r 0x73, 0, 0x11, x0, {}, x0" , in (reg) vaddr , options (nostack)) }
}

macro_rules! hfence_vvma_asid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hfence_vvma_asid in module {}", module_path!());
    };
}

mkfn!{
    hfence_vvma_asid_introspect!();
    # [doc = " Hypervisor memory management fence for given guest address space"] # [doc = ""] # [doc = " Guarantees that any previous stores already visible to the current hart are ordered before all"] # [doc = " implicit reads by that hart done for VS-stage address translation for instructions that:"] # [doc = " - are subsequent to the `HFENCE.VVMA`, and"] # [doc = " - execute when `hgatp.VMID` has the same setting as it did when `HFENCE.VVMA` executed."] # [doc = ""] # [doc = " This fence specifies a single guest address-space identifier."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hfence_vvma_asid (asid : usize) { asm ! (".insn r 0x73, 0, 0x11, x0, x0, {}" , in (reg) asid , options (nostack)) }
}

macro_rules! hfence_vvma_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hfence_vvma_all in module {}", module_path!());
    };
}

mkfn!{
    hfence_vvma_all_introspect!();
    # [doc = " Hypervisor memory management fence for all guest address spaces and guest virtual addresses"] # [doc = ""] # [doc = " Guarantees that any previous stores already visible to the current hart are ordered before all"] # [doc = " implicit reads by that hart done for VS-stage address translation for instructions that:"] # [doc = " - are subsequent to the `HFENCE.VVMA`, and"] # [doc = " - execute when `hgatp.VMID` has the same setting as it did when `HFENCE.VVMA` executed."] # [doc = ""] # [doc = " This fence applies to any guest address spaces and guest virtual addresses."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hfence_vvma_all () { asm ! (".insn r 0x73, 0, 0x11, x0, x0, x0" , options (nostack)) }
}

macro_rules! hfence_gvma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hfence_gvma in module {}", module_path!());
    };
}

mkfn!{
    hfence_gvma_introspect!();
    # [doc = " Hypervisor memory management fence for guest physical address and virtual machine"] # [doc = ""] # [doc = " Guarantees that any previous stores already visible to the current hart are ordered before all implicit reads"] # [doc = " by that hart done for G-stage address translation for instructions that follow the HFENCE.GVMA."] # [doc = ""] # [doc = " This fence specifies a single guest physical address, **shifted right by 2 bits**, and a single virtual machine"] # [doc = " by virtual machine identifier (VMID)."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hfence_gvma (gaddr : usize , vmid : usize) { asm ! (".insn r 0x73, 0, 0x31, x0, {}, {}" , in (reg) gaddr , in (reg) vmid , options (nostack)) }
}

macro_rules! hfence_gvma_gaddr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hfence_gvma_gaddr in module {}", module_path!());
    };
}

mkfn!{
    hfence_gvma_gaddr_introspect!();
    # [doc = " Hypervisor memory management fence for guest physical address"] # [doc = ""] # [doc = " Guarantees that any previous stores already visible to the current hart are ordered before all implicit reads"] # [doc = " by that hart done for G-stage address translation for instructions that follow the HFENCE.GVMA."] # [doc = ""] # [doc = " This fence specifies a single guest physical address; **the physical address should be shifted right by 2 bits**."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hfence_gvma_gaddr (gaddr : usize) { asm ! (".insn r 0x73, 0, 0x31, x0, {}, x0" , in (reg) gaddr , options (nostack)) }
}

macro_rules! hfence_gvma_vmid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hfence_gvma_vmid in module {}", module_path!());
    };
}

mkfn!{
    hfence_gvma_vmid_introspect!();
    # [doc = " Hypervisor memory management fence for given virtual machine"] # [doc = ""] # [doc = " Guarantees that any previous stores already visible to the current hart are ordered before all implicit reads"] # [doc = " by that hart done for G-stage address translation for instructions that follow the HFENCE.GVMA."] # [doc = ""] # [doc = " This fence specifies a single virtual machine by virtual machine identifier (VMID)."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hfence_gvma_vmid (vmid : usize) { asm ! (".insn r 0x73, 0, 0x31, x0, x0, {}" , in (reg) vmid , options (nostack)) }
}

macro_rules! hfence_gvma_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hfence_gvma_all in module {}", module_path!());
    };
}

mkfn!{
    hfence_gvma_all_introspect!();
    # [doc = " Hypervisor memory management fence for all virtual machines and guest physical addresses"] # [doc = ""] # [doc = " Guarantees that any previous stores already visible to the current hart are ordered before all implicit reads"] # [doc = " by that hart done for G-stage address translation for instructions that follow the HFENCE.GVMA."] # [doc = ""] # [doc = " This fence specifies all guest physical addresses and all virtual machines."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hfence_gvma_all () { asm ! (".insn r 0x73, 0, 0x31, x0, x0, x0" , options (nostack)) }
}

macro_rules! hinval_vvma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hinval_vvma in module {}", module_path!());
    };
}

mkfn!{
    hinval_vvma_introspect!();
    # [doc = " Invalidate hypervisor translation cache for given guest virtual address and guest address space"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `HFENCE.VVMA` instruction with the same values of `vaddr` and `asid` would invalidate."] # [doc = ""] # [doc = " This fence specifies a single guest virtual address, and a single guest address-space identifier."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hinval_vvma (vaddr : usize , asid : usize) { asm ! (".insn r 0x73, 0, 0x13, x0, {}, {}" , in (reg) vaddr , in (reg) asid , options (nostack)) }
}

macro_rules! hinval_vvma_vaddr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hinval_vvma_vaddr in module {}", module_path!());
    };
}

mkfn!{
    hinval_vvma_vaddr_introspect!();
    # [doc = " Invalidate hypervisor translation cache for given guest virtual address"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `HFENCE.VVMA` instruction with the same values of `vaddr` and `asid` would invalidate."] # [doc = ""] # [doc = " This fence specifies a single guest virtual address."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hinval_vvma_vaddr (vaddr : usize) { asm ! (".insn r 0x73, 0, 0x13, x0, {}, x0" , in (reg) vaddr , options (nostack)) }
}

macro_rules! hinval_vvma_asid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hinval_vvma_asid in module {}", module_path!());
    };
}

mkfn!{
    hinval_vvma_asid_introspect!();
    # [doc = " Invalidate hypervisor translation cache for given guest address space"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `HFENCE.VVMA` instruction with the same values of `vaddr` and `asid` would invalidate."] # [doc = ""] # [doc = " This fence specifies a single guest address-space identifier."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hinval_vvma_asid (asid : usize) { asm ! (".insn r 0x73, 0, 0x13, x0, x0, {}" , in (reg) asid , options (nostack)) }
}

macro_rules! hinval_vvma_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hinval_vvma_all in module {}", module_path!());
    };
}

mkfn!{
    hinval_vvma_all_introspect!();
    # [doc = " Invalidate hypervisor translation cache for all guest address spaces and guest virtual addresses"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `HFENCE.VVMA` instruction with the same values of `vaddr` and `asid` would invalidate."] # [doc = ""] # [doc = " This fence applies to any guest address spaces and guest virtual addresses."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hinval_vvma_all () { asm ! (".insn r 0x73, 0, 0x13, x0, x0, x0" , options (nostack)) }
}

macro_rules! hinval_gvma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hinval_gvma in module {}", module_path!());
    };
}

mkfn!{
    hinval_gvma_introspect!();
    # [doc = " Invalidate hypervisor translation cache for guest physical address and virtual machine"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `HFENCE.GVMA` instruction with the same values of `gaddr` and `vmid` would invalidate."] # [doc = ""] # [doc = " This fence specifies a single guest physical address, **shifted right by 2 bits**, and a single virtual machine"] # [doc = " by virtual machine identifier (VMID)."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hinval_gvma (gaddr : usize , vmid : usize) { asm ! (".insn r 0x73, 0, 0x33, x0, {}, {}" , in (reg) gaddr , in (reg) vmid , options (nostack)) }
}

macro_rules! hinval_gvma_gaddr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hinval_gvma_gaddr in module {}", module_path!());
    };
}

mkfn!{
    hinval_gvma_gaddr_introspect!();
    # [doc = " Invalidate hypervisor translation cache for guest physical address"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `HFENCE.GVMA` instruction with the same values of `gaddr` and `vmid` would invalidate."] # [doc = ""] # [doc = " This fence specifies a single guest physical address; **the physical address should be shifted right by 2 bits**."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hinval_gvma_gaddr (gaddr : usize) { asm ! (".insn r 0x73, 0, 0x33, x0, {}, x0" , in (reg) gaddr , options (nostack)) }
}

macro_rules! hinval_gvma_vmid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hinval_gvma_vmid in module {}", module_path!());
    };
}

mkfn!{
    hinval_gvma_vmid_introspect!();
    # [doc = " Invalidate hypervisor translation cache for given virtual machine"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `HFENCE.GVMA` instruction with the same values of `gaddr` and `vmid` would invalidate."] # [doc = ""] # [doc = " This fence specifies a single virtual machine by virtual machine identifier (VMID)."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hinval_gvma_vmid (vmid : usize) { asm ! (".insn r 0x73, 0, 0x33, x0, x0, {}" , in (reg) vmid , options (nostack)) }
}

macro_rules! hinval_gvma_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hinval_gvma_all in module {}", module_path!());
    };
}

mkfn!{
    hinval_gvma_all_introspect!();
    # [doc = " Invalidate hypervisor translation cache for all virtual machines and guest physical addresses"] # [doc = ""] # [doc = " This instruction invalidates any address-translation cache entries that an"] # [doc = " `HFENCE.GVMA` instruction with the same values of `gaddr` and `vmid` would invalidate."] # [doc = ""] # [doc = " This fence specifies all guest physical addresses and all virtual machines."] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub unsafe fn hinval_gvma_all () { asm ! (".insn r 0x73, 0, 0x33, x0, x0, x0" , options (nostack)) }
}

macro_rules! frrm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function frrm in module {}", module_path!());
    };
}

mkfn!{
    frrm_introspect!();
    # [doc = " Reads the floating-point rounding mode register `frm`"] # [doc = ""] # [doc = " According to \"F\" Standard Extension for Single-Precision Floating-Point, Version 2.2,"] # [doc = " the rounding mode field is defined as listed in the table below:"] # [doc = ""] # [doc = " | Rounding Mode | Mnemonic | Meaning |"] # [doc = " |:-------------|:----------|:---------|"] # [doc = " | 000 | RNE | Round to Nearest, ties to Even |"] # [doc = " | 001 | RTZ | Round towards Zero |"] # [doc = " | 010 | RDN | Round Down (towards −∞) |"] # [doc = " | 011 | RUP | Round Up (towards +∞) |"] # [doc = " | 100 | RMM | Round to Nearest, ties to Max Magnitude |"] # [doc = " | 101 |     | _Reserved for future use._ |"] # [doc = " | 110 |     | _Reserved for future use._ |"] # [doc = " | 111 | DYN | In Rounding Mode register, _reserved_. |"] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn frrm () -> u32 { let value : u32 ; unsafe { asm ! ("frrm {}" , out (reg) value , options (nomem , nostack)) } ; value }
}