macro_rules! TomlLockfileSourceIdErrorKind {
    () => {
        # [non_exhaustive] # [derive (Debug , thiserror :: Error)] enum TomlLockfileSourceIdErrorKind { # [error ("invalid source `{0}`")] InvalidSource (String) , # [error ("invalid url `{url}`: {msg}")] InvalidUrl { url : String , msg : String } , # [error ("unsupported source protocol: {0}")] UnsupportedSource (String) , }
    };
}

TomlLockfileSourceIdErrorKind!()