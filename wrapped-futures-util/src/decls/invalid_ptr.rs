macro_rules! invalid_ptr {
    () => {
        # [allow (clippy :: useless_transmute)] # [inline] fn invalid_ptr < T > (addr : usize) -> * mut T { unsafe { core :: mem :: transmute (addr) } }
    };
}

invalid_ptr!();