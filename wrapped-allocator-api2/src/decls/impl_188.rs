macro_rules! deps {
    () => {
        Vec!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl From < & str > for Vec < u8 > { # [doc = " Allocate a `Vec<u8>` and fill it with a UTF-8 string."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::{vec, vec::Vec};"] # [doc = ""] # [doc = " assert_eq!(Vec::from(\"123\"), vec![b'1', b'2', b'3']);"] # [doc = " ```"] # [inline (always)] fn from (s : & str) -> Vec < u8 > { From :: from (s . as_bytes ()) } }
    };
}

impl_188!();