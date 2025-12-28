macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! AnnotatedCommit {
    () => {
        deps!();
        # [doc = " A structure to represent an annotated commit, the input to merge and rebase."] # [doc = ""] # [doc = " An annotated commit contains information about how it was looked up, which"] # [doc = " may be useful for functions like merge or rebase to provide context to the"] # [doc = " operation."] pub struct AnnotatedCommit < 'repo > { raw : * mut raw :: git_annotated_commit , _marker : marker :: PhantomData < Commit < 'repo > > , }
    };
}

AnnotatedCommit!()