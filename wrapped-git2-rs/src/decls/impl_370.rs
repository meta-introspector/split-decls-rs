macro_rules! deps {
    () => {
        Error!();
        EmailCreateOptions!();
        Diff!();
        Signature!();
        Oid!();
        Email!();
        Buf!();
        IntoCString!();
        Commit!();
        Binding!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        impl Email { # [doc = " Returns a byte slice with stored e-mail patch in. `Email` could be"] # [doc = " created by one of the `from_*` functions."] pub fn as_slice (& self) -> & [u8] { & self . buf } # [doc = " Create a diff for a commit in mbox format for sending via email."] pub fn from_diff < T : IntoCString > (diff : & Diff < '_ > , patch_idx : usize , patch_count : usize , commit_id : & Oid , summary : T , body : T , author : & Signature < '_ > , opts : & mut EmailCreateOptions ,) -> Result < Self , Error > { let buf = Buf :: new () ; let summary = summary . into_c_string () ? ; let body = body . into_c_string () ? ; unsafe { try_call ! (raw :: git_email_create_from_diff (buf . raw () , Binding :: raw (diff) , patch_idx , patch_count , Binding :: raw (commit_id) , summary . as_ptr () , body . as_ptr () , Binding :: raw (author) , opts . raw ())) ; Ok (Self { buf }) } } # [doc = " Create a diff for a commit in mbox format for sending via email."] # [doc = " The commit must not be a merge commit."] pub fn from_commit (commit : & Commit < '_ > , opts : & mut EmailCreateOptions) -> Result < Self , Error > { let buf = Buf :: new () ; unsafe { try_call ! (raw :: git_email_create_from_commit (buf . raw () , commit . raw () , opts . raw ())) ; Ok (Self { buf }) } } }
    };
}

impl_370!();