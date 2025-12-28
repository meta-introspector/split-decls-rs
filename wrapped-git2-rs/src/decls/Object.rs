macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! Object {
    () => {
        deps!();
        # [doc = " A structure to represent a git [object][1]"] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects"] pub struct Object < 'repo > { raw : * mut raw :: git_object , _marker : marker :: PhantomData < & 'repo Repository > , }
    };
}

Object!()