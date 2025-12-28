macro_rules! position {
    () => {
        # [inline (always)] pub fn position (haystack : & [u16] , needle : u16) -> Option < usize > { haystack . iter () . position (| & x | x == needle) }
    };
}

position!();