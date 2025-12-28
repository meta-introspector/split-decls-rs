macro_rules! Repository {
    () => {
        # [doc = " An owned git repository, representing all state associated with the"] # [doc = " underlying filesystem."] # [doc = ""] # [doc = " This structure corresponds to a `git_repository` in libgit2. Many other"] # [doc = " types in git2-rs are derivative from this structure and are attached to its"] # [doc = " lifetime."] # [doc = ""] # [doc = " When a repository goes out of scope it is freed in memory but not deleted"] # [doc = " from the filesystem."] pub struct Repository { raw : * mut raw :: git_repository , }
    };
}

Repository!()