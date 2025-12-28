macro_rules! StreamKind {
    () => {
        # [doc = " The kind of stream to use for auto-configuration."] pub enum StreamKind { # [doc = " Standard output"] Stdout , # [doc = " Standard error"] Stderr , }
    };
}

StreamKind!();