macro_rules! extend {
    () => {
        # [cfg (feature = "http1")] pub (crate) fn extend (dst : & mut Vec < u8 >) { CACHED . with (| cache | { dst . extend_from_slice (cache . borrow () . buffer ()) ; }) }
    };
}

extend!();