macro_rules! deps {
    () => {
        RareByteOffset!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        impl RareByteOffset { # [doc = " Create a new rare byte offset. If the given offset is too big, then"] # [doc = " None is returned. In that case, callers should render the rare bytes"] # [doc = " prefilter inert."] fn new (max : usize) -> Option < RareByteOffset > { if max > u8 :: MAX as usize { None } else { Some (RareByteOffset { max : max as u8 }) } } }
    };
}

impl_388!()