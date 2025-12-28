macro_rules! cast_ptr {
    () => {
        # [inline] pub (crate) const fn cast_ptr < T > (n : & T) -> * const T { n }
    };
}

cast_ptr!()