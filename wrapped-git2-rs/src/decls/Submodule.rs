macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! Submodule {
    () => {
        deps!();
        # [doc = " A structure to represent a git [submodule][1]"] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Tools-Submodules"] pub struct Submodule < 'repo > { raw : * mut raw :: git_submodule , _marker : marker :: PhantomData < & 'repo Repository > , }
    };
}

Submodule!()