macro_rules! deps {
    () => {
        Hash234!();
    };
}

macro_rules! Bt4 {
    () => {
        deps!();
        # [doc = " Binary Tree with 4-byte matching"] pub (crate) struct Bt4 { hash : Hash234 , tree : Vec < i32 > , depth_limit : i32 , cyclic_size : i32 , cyclic_pos : i32 , lz_pos : i32 , }
    };
}

Bt4!();