macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! Blob {
    () => {
        deps!();
        # [doc = " A structure to represent a git [blob][1]"] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects"] pub struct Blob < 'repo > { raw : * mut raw :: git_blob , _marker : marker :: PhantomData < Object < 'repo > > , }
    };
}

Blob!()