macro_rules! invalid_mut {
    () => {
        # [cfg (feature = "alloc")] # [inline (always)] fn invalid_mut < T > (addr : usize) -> * mut T { # [allow (clippy :: useless_transmute , clippy :: transmutes_expressible_as_ptr_casts)] unsafe { core :: mem :: transmute (addr) } }
    };
}

invalid_mut!()