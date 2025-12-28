macro_rules! E_BOUNDS {
    () => {
        # [cfg (feature = "std")] const E_BOUNDS : windows_core :: HRESULT = windows_core :: HRESULT (0x8000000B_u32 as _) ;
    };
}

E_BOUNDS!();