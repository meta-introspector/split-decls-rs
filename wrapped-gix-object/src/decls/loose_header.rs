macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! loose_header {
    () => {
        deps!();
        # [doc = " Generates a loose header buffer"] pub fn loose_header (kind : crate :: Kind , size : u64) -> smallvec :: SmallVec < u8 , 28 > { let mut v = smallvec :: SmallVec :: new () ; v . extend_from_slice (kind . as_bytes ()) ; v . extend_from_slice (SPACE) ; v . extend_from_slice (itoa :: Buffer :: new () . format (size) . as_bytes ()) ; v . extend_from_slice (b"\0") ; v }
    };
}

loose_header!()