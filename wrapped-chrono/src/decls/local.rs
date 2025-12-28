macro_rules! local {
    () => {
        # [cfg (feature = "clock")] pub (crate) mod local ;
    };
}

local!();