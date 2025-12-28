macro_rules! deps {
    () => {
        MergeHole!();
        SendPtr!();
        Chunks!();
    };
}

macro_rules! merge_recurse {
    () => {
        deps!();
        # [doc = " Recursively merges pre-sorted chunks inside `v`."] # [doc = ""] # [doc = " Chunks of `v` are stored in `chunks` as intervals (inclusive left and exclusive right bound)."] # [doc = " Argument `buf` is an auxiliary buffer that will be used during the procedure."] # [doc = " If `into_buf` is true, the result will be stored into `buf`, otherwise it will be in `v`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The number of chunks must be positive and they must be adjacent: the right bound of each chunk"] # [doc = " must equal the left bound of the following chunk."] # [doc = ""] # [doc = " The buffer must be at least as long as `v`."] unsafe fn merge_recurse < T , F > (v : * mut T , buf : * mut T , chunks : & [(usize , usize)] , into_buf : bool , is_less : & F ,) where T : Send , F : Fn (& T , & T) -> bool + Sync , { unsafe { let len = chunks . len () ; debug_assert ! (len > 0) ; if len == 1 { if into_buf { let (start , end) = chunks [0] ; let src = v . add (start) ; let dest = buf . add (start) ; ptr :: copy_nonoverlapping (src , dest , end - start) ; } return ; } let (start , _) = chunks [0] ; let (mid , _) = chunks [len / 2] ; let (_ , end) = chunks [len - 1] ; let (left , right) = chunks . split_at (len / 2) ; let (src , dest) = if into_buf { (v , buf) } else { (buf , v) } ; let guard = MergeHole { start : src . add (start) , end : src . add (end) , dest : dest . add (start) , } ; let v = SendPtr (v) ; let buf = SendPtr (buf) ; rayon_core :: join (move | | merge_recurse (v . get () , buf . get () , left , ! into_buf , is_less) , move | | merge_recurse (v . get () , buf . get () , right , ! into_buf , is_less) ,) ; mem :: forget (guard) ; let src_left = slice :: from_raw_parts_mut (src . add (start) , mid - start) ; let src_right = slice :: from_raw_parts_mut (src . add (mid) , end - mid) ; par_merge (src_left , src_right , dest . add (start) , is_less) ; } }
    };
}

merge_recurse!();