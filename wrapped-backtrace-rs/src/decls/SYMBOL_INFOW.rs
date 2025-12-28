macro_rules! deps {
    () => {
        SYMBOL_INFO_FLAGS!();
    };
}

macro_rules! SYMBOL_INFOW {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct SYMBOL_INFOW { pub SizeOfStruct : u32 , pub TypeIndex : u32 , pub Reserved : [u64 ; 2] , pub Index : u32 , pub Size : u32 , pub ModBase : u64 , pub Flags : SYMBOL_INFO_FLAGS , pub Value : u64 , pub Address : u64 , pub Register : u32 , pub Scope : u32 , pub Tag : u32 , pub NameLen : u32 , pub MaxNameLen : u32 , pub Name : [u16 ; 1] , }
    };
}

SYMBOL_INFOW!();