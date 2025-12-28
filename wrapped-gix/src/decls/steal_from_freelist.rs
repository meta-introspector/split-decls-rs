macro_rules! steal_from_freelist {
    () => {
        # [doc = " In conjunction with the handles free list, leaving an empty Vec in place of the original causes it to not be"] # [doc = " returned to the free list."] fn steal_from_freelist (data : & mut Vec < u8 >) -> Vec < u8 > { std :: mem :: take (data) }
    };
}

steal_from_freelist!()