macro_rules! macro_69 {
    () => {
        cfg_if :: cfg_if ! { if # [cfg (feature = "std")] { extern crate std as alloc ; } else { extern crate alloc ; } }
    };
}

macro_69!();