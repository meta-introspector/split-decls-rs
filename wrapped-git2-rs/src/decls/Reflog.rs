macro_rules! Reflog {
    () => {
        # [doc = " A reference log of a git repository."] pub struct Reflog { raw : * mut raw :: git_reflog , }
    };
}

Reflog!()