macro_rules! deps {
    () => {
        UnblamedHunk!();
    };
}

macro_rules! pass_blame_from_to {
    () => {
        deps!();
        # [doc = " Pass ownership of each unblamed hunk of `from` to `to`."] # [doc = ""] # [doc = " This happens when `from` didn't actually change anything in the blamed file."] fn pass_blame_from_to (from : ObjectId , to : ObjectId , hunks_to_blame : & mut Vec < UnblamedHunk >) { for unblamed_hunk in hunks_to_blame { unblamed_hunk . pass_blame (from , to) ; } }
    };
}

pass_blame_from_to!();