macro_rules! deps {
    () => {
        ItemSliceSync!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        # [allow (unsafe_code)] unsafe impl < T > Sync for ItemSliceSync < '_ , T > where T : Send { }
    };
}

impl_55!();