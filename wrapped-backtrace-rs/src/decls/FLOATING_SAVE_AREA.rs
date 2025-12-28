macro_rules! FLOATING_SAVE_AREA {
    () => {
        # [repr (C)] # [cfg (target_arch = "x86")] # [derive (Clone , Copy)] pub struct FLOATING_SAVE_AREA { pub ControlWord : u32 , pub StatusWord : u32 , pub TagWord : u32 , pub ErrorOffset : u32 , pub ErrorSelector : u32 , pub DataOffset : u32 , pub DataSelector : u32 , pub RegisterArea : [u8 ; 80] , pub Spare0 : u32 , }
    };
}

FLOATING_SAVE_AREA!();