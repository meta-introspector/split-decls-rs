macro_rules! StringArray {
    () => {
        # [doc = " A string array structure used by libgit2"] # [doc = ""] # [doc = " Some APIs return arrays of strings which originate from libgit2. This"] # [doc = " wrapper type behaves a little like `Vec<&str>` but does so without copying"] # [doc = " the underlying strings until necessary."] pub struct StringArray { raw : raw :: git_strarray , }
    };
}

StringArray!();