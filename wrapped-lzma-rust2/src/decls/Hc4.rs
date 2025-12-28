macro_rules! deps {
    () => {
        Hash234!();
    };
}

macro_rules! Hc4 {
    () => {
        deps!();
        # [doc = " Hash Chain with 4-byte matching"] pub (crate) struct Hc4 { hash : Hash234 , chain : Vec < i32 > , depth_limit : i32 , cyclic_size : i32 , cyclic_pos : i32 , lz_pos : i32 , }
    };
}

Hc4!()