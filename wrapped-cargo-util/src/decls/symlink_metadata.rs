macro_rules! symlink_metadata {
    () => {
        # [doc = " Returns metadata for a file without following symlinks."] # [doc = ""] # [doc = " Equivalent to [`std::fs::metadata`] with better error messages."] pub fn symlink_metadata < P : AsRef < Path > > (path : P) -> Result < Metadata > { let path = path . as_ref () ; std :: fs :: symlink_metadata (path) . with_context (| | format ! ("failed to load metadata for path `{}`" , path . display ())) }
    };
}

symlink_metadata!();