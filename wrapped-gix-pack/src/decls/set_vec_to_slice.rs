macro_rules! set_vec_to_slice {
    () => {
        # [doc = " Replaces content of the given `Vec` with the slice. The vec will have the same length"] # [doc = " as the slice. The vec can be either `&mut Vec` or `Vec`."] # [doc = " Returns `None` if no memory could be allocated."] # [cfg (any (feature = "pack-cache-lru-static" , feature = "pack-cache-lru-dynamic" , feature = "object-cache-dynamic"))] fn set_vec_to_slice < V : std :: borrow :: BorrowMut < Vec < u8 > > > (mut vec : V , source : & [u8]) -> Option < V > { let out = vec . borrow_mut () ; out . clear () ; out . try_reserve (source . len ()) . ok () ? ; out . extend_from_slice (source) ; Some (vec) }
    };
}

set_vec_to_slice!()