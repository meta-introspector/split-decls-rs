macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! Tree {
    () => {
        deps!();
        # [doc = " A structure to represent a git [tree][1]"] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects"] pub struct Tree < 'repo > { raw : * mut raw :: git_tree , _marker : marker :: PhantomData < Object < 'repo > > , }
    };
}

Tree!();