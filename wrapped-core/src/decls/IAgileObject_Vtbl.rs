macro_rules! deps {
    () => {
        IUnknown_Vtbl!();
    };
}

macro_rules! IAgileObject_Vtbl {
    () => {
        deps!();
        # [repr (C)] # [doc (hidden)] pub struct IAgileObject_Vtbl { pub base__ : windows_core :: IUnknown_Vtbl , }
    };
}

IAgileObject_Vtbl!()