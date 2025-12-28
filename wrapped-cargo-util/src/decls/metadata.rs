macro_rules! metadata {
    () => {
        # [doc = " Returns metadata for a file (follows symlinks)."] # [doc = ""] # [doc = " Equivalent to [`std::fs::metadata`] with better error messages."] pub fn metadata < P : AsRef < Path > > (path : P) -> Result < Metadata > { let path = path . as_ref () ; std :: fs :: metadata (path) . with_context (| | format ! ("failed to load metadata for path `{}`" , path . display ())) }
    };
}

metadata!()