macro_rules! xor_slices {
    () => {
        # [doc = " xor_slices!(src, destination): XOR $src into $destination slice."] # [doc = " Uses iter() and .zip(), so it short-circuits on the slice that has"] # [doc = " the smallest length."] macro_rules ! xor_slices { ($ src : expr , $ destination : expr) => { for (inplace , _src_elem) in $ destination . iter_mut () . zip ($ src . iter ()) { * inplace ^= _src_elem ; } } ; }
    };
}

xor_slices!();