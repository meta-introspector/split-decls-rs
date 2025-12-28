macro_rules! deps {
    () => {
        ItemSliceSync!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        # [allow (unsafe_code)] unsafe impl < T > Send for ItemSliceSync < '_ , T > where T : Send { }
    };
}

impl_54!();