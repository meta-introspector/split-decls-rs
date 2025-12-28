macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! Remote {
    () => {
        deps!();
        # [doc = " A structure representing a [remote][1] of a git repository."] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Basics-Working-with-Remotes"] # [doc = ""] # [doc = " The lifetime is the lifetime of the repository that it is attached to. The"] # [doc = " remote is used to manage fetches and pushes as well as refspecs."] pub struct Remote < 'repo > { raw : * mut raw :: git_remote , _marker : marker :: PhantomData < & 'repo Repository > , }
    };
}

Remote!()