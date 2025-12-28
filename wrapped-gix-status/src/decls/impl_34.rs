macro_rules! deps {
    () => {
        HashEq!();
        Entry!();
        CompareBlobs!();
        ReadData!();
        FastEq!();
        Error!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl CompareBlobs for FastEq { type Output = () ; fn compare_blobs < 'a , 'b > (& mut self , entry : & Entry , worktree_file_size : u64 , data : impl ReadData < 'a > , buf : & mut Vec < u8 > ,) -> Result < Option < Self :: Output > , Error > { if u64 :: from (entry . stat . size) != worktree_file_size && (entry . id . is_empty_blob () || entry . stat . size != 0) { return Ok (Some (())) ; } HashEq . compare_blobs (entry , worktree_file_size , data , buf) . map (| opt | opt . map (| _ | ())) } }
    };
}

impl_34!()