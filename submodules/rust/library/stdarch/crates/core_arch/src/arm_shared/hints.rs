
macro_rules! __wfi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __wfi in module {}", module_path!());
    };
}

mkfn!{
    __wfi_introspect!();
    # [doc = " Generates a WFI (wait for interrupt) hint instruction, or nothing."] # [doc = ""] # [doc = " The WFI instruction allows (but does not require) the processor to enter a"] # [doc = " low-power state until one of a number of asynchronous events occurs."] # [cfg (any (target_feature = "v6" , target_arch = "aarch64" , target_arch = "arm64ec" , doc))] # [inline (always)] # [unstable (feature = "stdarch_arm_hints" , issue = "117218")] pub unsafe fn __wfi () { hint (HINT_WFI) ; }
}

macro_rules! __wfe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __wfe in module {}", module_path!());
    };
}

mkfn!{
    __wfe_introspect!();
    # [doc = " Generates a WFE (wait for event) hint instruction, or nothing."] # [doc = ""] # [doc = " The WFE instruction allows (but does not require) the processor to enter a"] # [doc = " low-power state until some event occurs such as a SEV being issued by"] # [doc = " another processor."] # [cfg (any (target_feature = "v6" , target_arch = "aarch64" , target_arch = "arm64ec" , doc))] # [inline (always)] # [unstable (feature = "stdarch_arm_hints" , issue = "117218")] pub unsafe fn __wfe () { hint (HINT_WFE) ; }
}

macro_rules! __sev_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __sev in module {}", module_path!());
    };
}

mkfn!{
    __sev_introspect!();
    # [doc = " Generates a SEV (send a global event) hint instruction."] # [doc = ""] # [doc = " This causes an event to be signaled to all processors in a multiprocessor"] # [doc = " system. It is a NOP on a uniprocessor system."] # [cfg (any (target_feature = "v6" , target_arch = "aarch64" , target_arch = "arm64ec" , doc))] # [inline (always)] # [unstable (feature = "stdarch_arm_hints" , issue = "117218")] pub unsafe fn __sev () { hint (HINT_SEV) ; }
}

macro_rules! __sevl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __sevl in module {}", module_path!());
    };
}

mkfn!{
    __sevl_introspect!();
    # [doc = " Generates a send a local event hint instruction."] # [doc = ""] # [doc = " This causes an event to be signaled to only the processor executing this"] # [doc = " instruction. In a multiprocessor system, it is not required to affect the"] # [doc = " other processors."] # [cfg (any (target_feature = "v8" , target_arch = "aarch64" , target_arch = "arm64ec" , doc ,))] # [inline (always)] # [unstable (feature = "stdarch_arm_hints" , issue = "117218")] pub unsafe fn __sevl () { hint (HINT_SEVL) ; }
}

macro_rules! __yield_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __yield in module {}", module_path!());
    };
}

mkfn!{
    __yield_introspect!();
    # [doc = " Generates a YIELD hint instruction."] # [doc = ""] # [doc = " This enables multithreading software to indicate to the hardware that it is"] # [doc = " performing a task, for example a spin-lock, that could be swapped out to"] # [doc = " improve overall system performance."] # [cfg (any (target_feature = "v6" , target_arch = "aarch64" , target_arch = "arm64ec" , doc))] # [inline (always)] # [unstable (feature = "stdarch_arm_hints" , issue = "117218")] pub unsafe fn __yield () { hint (HINT_YIELD) ; }
}

macro_rules! __nop_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __nop in module {}", module_path!());
    };
}

mkfn!{
    __nop_introspect!();
    # [doc = " Generates an unspecified no-op instruction."] # [doc = ""] # [doc = " Note that not all architectures provide a distinguished NOP instruction. On"] # [doc = " those that do, it is unspecified whether this intrinsic generates it or"] # [doc = " another instruction. It is not guaranteed that inserting this instruction"] # [doc = " will increase execution time."] # [inline (always)] # [unstable (feature = "stdarch_arm_hints" , issue = "117218")] pub unsafe fn __nop () { crate :: arch :: asm ! ("nop" , options (nomem , nostack , preserves_flags)) ; }
}
mkitem!{unsafe extern "unadjusted" { # [cfg_attr (any (target_arch = "aarch64" , target_arch = "arm64ec") , link_name = "llvm.aarch64.hint")] # [cfg_attr (target_arch = "arm" , link_name = "llvm.arm.hint")] fn hint (_ : i32) ; }}
mkitem!{const HINT_NOP : i32 = 0 ;}
mkitem!{const HINT_YIELD : i32 = 1 ;}
mkitem!{const HINT_WFE : i32 = 2 ;}
mkitem!{const HINT_WFI : i32 = 3 ;}
mkitem!{const HINT_SEV : i32 = 4 ;}
mkitem!{const HINT_SEVL : i32 = 5 ;}