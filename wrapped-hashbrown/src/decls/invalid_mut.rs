macro_rules! invalid_mut {
    () => {
        # [inline (always)] # [allow (clippy :: useless_transmute)] pub (crate) fn invalid_mut < T > (addr : usize) -> * mut T { unsafe { core :: mem :: transmute (addr) } }
    };
}

invalid_mut!();