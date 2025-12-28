macro_rules! deps {
    () => {
        UnblamedHunk!();
        BlameEntry!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl BlameEntry { # [doc = " Create an offset from a portion of the *Blamed File*."] fn from_unblamed_hunk (unblamed_hunk : & UnblamedHunk , commit_id : ObjectId) -> Option < Self > { let range_in_source_file = unblamed_hunk . get_range (& commit_id) ? ; Some (Self { start_in_blamed_file : unblamed_hunk . range_in_blamed_file . start , start_in_source_file : range_in_source_file . start , len : force_non_zero (range_in_source_file . len () as u32) , commit_id , source_file_name : unblamed_hunk . source_file_name . clone () , }) } }
    };
}

impl_43!()