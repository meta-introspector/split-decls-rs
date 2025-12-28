macro_rules! deps {
    () => {
        Refdb!();
    };
}

macro_rules! Reference {
    () => {
        deps!();
        # [doc = " A structure to represent a git [reference][1]."] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Internals-Git-References"] pub struct Reference < 'repo > { raw : * mut raw :: git_reference , _marker : marker :: PhantomData < Refdb < 'repo > > , }
    };
}

Reference!();