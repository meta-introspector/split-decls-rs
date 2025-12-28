macro_rules! memmem {
    () => {
        # [doc = " Find the offset of the first occurrence of needle in the data."] # [doc = ""] # [doc = " The offset must have the given alignment."] fn memmem (data : & [u8] , needle : & [u8] , align : usize) -> Option < usize > { let mut offset = 0 ; loop { if data . get (offset ..) ? . get (.. needle . len ()) ? == needle { return Some (offset) ; } offset += align ; } }
    };
}

memmem!()