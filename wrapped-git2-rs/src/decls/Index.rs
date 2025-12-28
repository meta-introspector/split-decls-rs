macro_rules! Index {
    () => {
        # [doc = " A structure to represent a git [index][1]"] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects"] pub struct Index { raw : * mut raw :: git_index , }
    };
}

Index!();