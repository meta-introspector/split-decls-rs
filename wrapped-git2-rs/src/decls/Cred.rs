macro_rules! Cred {
    () => {
        # [doc = " A structure to represent git credentials in libgit2."] pub struct Cred { raw : * mut raw :: git_cred , }
    };
}

Cred!();