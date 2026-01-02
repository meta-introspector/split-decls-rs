mkmod!{cp15, { 
                getname!(cp15);
                getsrc!(cp15);
                getpath!(cp15);
                get_deps!(cp15);
                get_crates!(cp15);
                mkinclude!(cp15);
                 
            }}
mkuse!{# [cfg (not (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_feature = "v7" , target_feature = "mclass")))] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub use self :: cp15 :: * ;}
mkitem!{# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_feature = "v7" , target_feature = "mclass"))] macro_rules ! dmb_dsb { ($ A : ident) => { # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] impl super :: super :: sealed :: Dmb for $ A { # [inline (always)] unsafe fn __dmb (& self) { super :: dmb (super :: arg ::$ A) } } # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] impl super :: super :: sealed :: Dsb for $ A { # [inline (always)] unsafe fn __dsb (& self) { super :: dsb (super :: arg ::$ A) } } } ; }}
mkmod!{common, { 
                getname!(common);
                getsrc!(common);
                getpath!(common);
                get_deps!(common);
                get_crates!(common);
                mkinclude!(common);
                 
            }}
mkuse!{# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_feature = "v7" , target_feature = "mclass"))] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub use self :: common :: * ;}
mkmod!{not_mclass, { 
                getname!(not_mclass);
                getsrc!(not_mclass);
                getpath!(not_mclass);
                get_deps!(not_mclass);
                get_crates!(not_mclass);
                mkinclude!(not_mclass);
                 
            }}
mkuse!{# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_feature = "v7" ,))] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub use self :: not_mclass :: * ;}
mkmod!{v8, { 
                getname!(v8);
                getsrc!(v8);
                getpath!(v8);
                get_deps!(v8);
                get_crates!(v8);
                mkinclude!(v8);
                 
            }}
mkuse!{# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec"))] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub use self :: v8 :: * ;}

macro_rules! __dmb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __dmb in module {}", module_path!());
    };
}

mkfn!{
    __dmb_introspect!();
    # [doc = " Generates a DMB (data memory barrier) instruction or equivalent CP15 instruction."] # [doc = ""] # [doc = " DMB ensures the observed ordering of memory accesses. Memory accesses of the specified type"] # [doc = " issued before the DMB are guaranteed to be observed (in the specified scope) before memory"] # [doc = " accesses issued after the DMB."] # [doc = ""] # [doc = " For example, DMB should be used between storing data, and updating a flag variable that makes"] # [doc = " that data available to another core."] # [doc = ""] # [doc = " The __dmb() intrinsic also acts as a compiler memory barrier of the appropriate type."] # [inline (always)] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub unsafe fn __dmb < A > (arg : A) where A : super :: sealed :: Dmb , { arg . __dmb () }
}

macro_rules! __dsb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __dsb in module {}", module_path!());
    };
}

mkfn!{
    __dsb_introspect!();
    # [doc = " Generates a DSB (data synchronization barrier) instruction or equivalent CP15 instruction."] # [doc = ""] # [doc = " DSB ensures the completion of memory accesses. A DSB behaves as the equivalent DMB and has"] # [doc = " additional properties. After a DSB instruction completes, all memory accesses of the specified"] # [doc = " type issued before the DSB are guaranteed to have completed."] # [doc = ""] # [doc = " The __dsb() intrinsic also acts as a compiler memory barrier of the appropriate type."] # [inline (always)] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub unsafe fn __dsb < A > (arg : A) where A : super :: sealed :: Dsb , { arg . __dsb () }
}

macro_rules! __isb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __isb in module {}", module_path!());
    };
}

mkfn!{
    __isb_introspect!();
    # [doc = " Generates an ISB (instruction synchronization barrier) instruction or equivalent CP15"] # [doc = " instruction."] # [doc = ""] # [doc = " This instruction flushes the processor pipeline fetch buffers, so that following instructions"] # [doc = " are fetched from cache or memory."] # [doc = ""] # [doc = " An ISB is needed after some system maintenance operations. An ISB is also needed before"] # [doc = " transferring control to code that has been loaded or modified in memory, for example by an"] # [doc = " overlay mechanism or just-in-time code generator.  (Note that if instruction and data caches are"] # [doc = " separate, privileged cache maintenance operations would be needed in order to unify the caches.)"] # [doc = ""] # [doc = " The only supported argument for the __isb() intrinsic is 15, corresponding to the SY (full"] # [doc = " system) scope of the ISB instruction."] # [inline (always)] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub unsafe fn __isb < A > (arg : A) where A : super :: sealed :: Isb , { arg . __isb () }
}
mkitem!{unsafe extern "unadjusted" { # [cfg_attr (any (target_arch = "aarch64" , target_arch = "arm64ec") , link_name = "llvm.aarch64.dmb")] # [cfg_attr (target_arch = "arm" , link_name = "llvm.arm.dmb")] fn dmb (_ : i32) ; # [cfg_attr (any (target_arch = "aarch64" , target_arch = "arm64ec") , link_name = "llvm.aarch64.dsb")] # [cfg_attr (target_arch = "arm" , link_name = "llvm.arm.dsb")] fn dsb (_ : i32) ; # [cfg_attr (any (target_arch = "aarch64" , target_arch = "arm64ec") , link_name = "llvm.aarch64.isb")] # [cfg_attr (target_arch = "arm" , link_name = "llvm.arm.isb")] fn isb (_ : i32) ; }}
mkmod!{arg, { 
                getname!(arg);
                getsrc!(arg);
                getpath!(arg);
                get_deps!(arg);
                get_crates!(arg);
                mkinclude!(arg);
                mkitem!{pub const SY : i32 = 15 ;}
mkitem!{pub const ST : i32 = 14 ;}
mkitem!{pub const LD : i32 = 13 ;}
mkitem!{pub const ISH : i32 = 11 ;}
mkitem!{pub const ISHST : i32 = 10 ;}
mkitem!{pub const ISHLD : i32 = 9 ;}
mkitem!{pub const NSH : i32 = 7 ;}
mkitem!{pub const NSHST : i32 = 6 ;}
mkitem!{pub const NSHLD : i32 = 5 ;}
mkitem!{pub const OSH : i32 = 3 ;}
mkitem!{pub const OSHST : i32 = 2 ;}
mkitem!{pub const OSHLD : i32 = 1 ;} 
            }}