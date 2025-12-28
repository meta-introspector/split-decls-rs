macro_rules! deps {
    () => {
        FLOATING_SAVE_AREA!();
        CONTEXT_FLAGS!();
    };
}

macro_rules! CONTEXT {
    () => {
        deps!();
        # [repr (C)] # [cfg (target_arch = "x86")] # [derive (Clone , Copy)] pub struct CONTEXT { pub ContextFlags : CONTEXT_FLAGS , pub Dr0 : u32 , pub Dr1 : u32 , pub Dr2 : u32 , pub Dr3 : u32 , pub Dr6 : u32 , pub Dr7 : u32 , pub FloatSave : FLOATING_SAVE_AREA , pub SegGs : u32 , pub SegFs : u32 , pub SegEs : u32 , pub SegDs : u32 , pub Edi : u32 , pub Esi : u32 , pub Ebx : u32 , pub Edx : u32 , pub Ecx : u32 , pub Eax : u32 , pub Ebp : u32 , pub Eip : u32 , pub SegCs : u32 , pub EFlags : u32 , pub Esp : u32 , pub SegSs : u32 , pub ExtendedRegisters : [u8 ; 512] , }
    };
}

CONTEXT!()