macro_rules! deps {
    () => {
        Binding!();
        Oid!();
        AnnotatedCommit!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl < 'repo > AnnotatedCommit < 'repo > { # [doc = " Gets the commit ID that the given git_annotated_commit refers to"] pub fn id (& self) -> Oid { unsafe { Binding :: from_raw (raw :: git_annotated_commit_id (self . raw)) } } # [doc = " Get the refname that the given git_annotated_commit refers to"] # [doc = ""] # [doc = " Returns None if it is not valid utf8"] pub fn refname (& self) -> Option < & str > { str :: from_utf8 (self . refname_bytes ()) . ok () } # [doc = " Get the refname that the given git_annotated_commit refers to."] pub fn refname_bytes (& self) -> & [u8] { unsafe { crate :: opt_bytes (self , raw :: git_annotated_commit_ref (& * self . raw)) . unwrap () } } }
    };
}

impl_424!();