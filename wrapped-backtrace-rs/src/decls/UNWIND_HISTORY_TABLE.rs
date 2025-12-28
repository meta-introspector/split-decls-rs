macro_rules! deps {
    () => {
        UNWIND_HISTORY_TABLE_ENTRY!();
    };
}

macro_rules! UNWIND_HISTORY_TABLE {
    () => {
        deps!();
        # [repr (C)] # [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub struct UNWIND_HISTORY_TABLE { pub Count : u32 , pub LocalHint : u8 , pub GlobalHint : u8 , pub Search : u8 , pub Once : u8 , pub LowAddress : usize , pub HighAddress : usize , pub Entry : [UNWIND_HISTORY_TABLE_ENTRY ; 12] , }
    };
}

UNWIND_HISTORY_TABLE!();