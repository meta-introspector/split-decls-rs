macro_rules! GUID {
    () => {
        pub struct GUID (pub u32 , pub u16 , pub u16 , pub u8 , pub u8 , pub u8 , pub u8 , pub u8 , pub u8 , pub u8 , pub u8 ,) ;
    };
}

GUID!();