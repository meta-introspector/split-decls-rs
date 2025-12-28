macro_rules! deps {
    () => {
        ConsumeBinaryHunkDelegate!();
        HunkHeader!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        # [doc = " An implementation that writes hunks into a hunman-readable byte buffer."] impl ConsumeBinaryHunkDelegate for BString { fn consume_binary_hunk (& mut self , _header : HunkHeader , header_str : & str , hunk : & [u8]) -> std :: io :: Result < () > { self . push_str (header_str) ; self . extend_from_slice (hunk) ; Ok (()) } }
    };
}

impl_139!()