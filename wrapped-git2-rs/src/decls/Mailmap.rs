macro_rules! Mailmap {
    () => {
        # [doc = " A structure to represent a repository's .mailmap file."] # [doc = ""] # [doc = " The representation cannot be written to disk."] pub struct Mailmap { raw : * mut raw :: git_mailmap , }
    };
}

Mailmap!()