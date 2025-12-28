macro_rules! deps {
    () => {
        ConsumeBinaryHunkDelegate!();
        HunkHeader!();
        Error!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        # [doc = " An implementation that fails if the input isn't UTF-8."] impl ConsumeBinaryHunkDelegate for String { fn consume_binary_hunk (& mut self , _header : HunkHeader , header_str : & str , hunk : & [u8]) -> std :: io :: Result < () > { self . push_str (header_str) ; self . push_str (hunk . to_str () . map_err (std :: io :: Error :: other) ?) ; Ok (()) } }
    };
}

impl_137!();