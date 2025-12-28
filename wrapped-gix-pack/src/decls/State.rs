macro_rules! deps {
    () => {
        Item!();
        ItemSliceSync!();
    };
}

macro_rules! State {
    () => {
        deps!();
        pub (super) struct State < 'items , F , MBFN , T : Send > { pub delta_bytes : Vec < u8 > , pub fully_resolved_delta_bytes : Vec < u8 > , pub progress : Box < dyn Progress > , pub resolve : F , pub modify_base : MBFN , pub child_items : & 'items ItemSliceSync < 'items , Item < T > > , }
    };
}

State!();