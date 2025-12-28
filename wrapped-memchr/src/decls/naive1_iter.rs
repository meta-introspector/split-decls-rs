macro_rules! naive1_iter {
    () => {
        # [cfg (not (miri))] pub (crate) fn naive1_iter < 'a > (n1 : u8 , haystack : & 'a [u8] ,) -> impl DoubleEndedIterator < Item = usize > + 'a { haystack . iter () . enumerate () . filter (move | & (_ , & b) | b == n1) . map (| t | t . 0) }
    };
}

naive1_iter!();