macro_rules! read_slice {
    () => {
        # [doc = " Copies a &[u8] $src into a &mut [$ty] $dst for the endianness given by"] # [doc = " $from_bytes (must be either from_be_bytes or from_le_bytes)."] # [doc = ""] # [doc = " Panics if $src.len() != $dst.len() * size_of::<$ty>()."] macro_rules ! read_slice { ($ src : expr , $ dst : expr , $ ty : ty , $ from_bytes : ident) => { { const SIZE : usize = core :: mem :: size_of ::<$ ty > () ; let src : & [u8] = $ src ; let dst : & mut [$ ty] = $ dst ; assert_eq ! (src . len () , dst . len () * SIZE) ; for (src , dst) in src . chunks_exact (SIZE) . zip (dst . iter_mut ()) { * dst = <$ ty >::$ from_bytes (src . try_into () . unwrap ()) ; } } } ; }
    };
}

read_slice!();