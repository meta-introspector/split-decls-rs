macro_rules! deps {
    () => {
        AnnotatedCommit!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl < 'repo > Drop for AnnotatedCommit < 'repo > { fn drop (& mut self) { unsafe { raw :: git_annotated_commit_free (self . raw) } } }
    };
}

impl_428!();