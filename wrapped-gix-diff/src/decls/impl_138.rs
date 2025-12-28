macro_rules! deps {
    () => {
        HunkHeader!();
        ConsumeBinaryHunkDelegate!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        # [doc = " An implementation that writes hunks into a byte buffer."] impl ConsumeBinaryHunkDelegate for Vec < u8 > { fn consume_binary_hunk (& mut self , _header : HunkHeader , header_str : & str , hunk : & [u8]) -> std :: io :: Result < () > { self . push_str (header_str) ; self . extend_from_slice (hunk) ; Ok (()) } }
    };
}

impl_138!();