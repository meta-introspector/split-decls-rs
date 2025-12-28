macro_rules! naive2_iter {
    () => {
        # [cfg (not (miri))] pub (crate) fn naive2_iter < 'a > (n1 : u8 , n2 : u8 , haystack : & 'a [u8] ,) -> impl DoubleEndedIterator < Item = usize > + 'a { haystack . iter () . enumerate () . filter (move | & (_ , & b) | b == n1 || b == n2) . map (| t | t . 0) }
    };
}

naive2_iter!()