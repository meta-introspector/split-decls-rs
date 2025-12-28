macro_rules! write {
    () => {
        # [doc = " Writes a file to disk."] # [doc = ""] # [doc = " Equivalent to [`std::fs::write`] with better error messages."] pub fn write < P : AsRef < Path > , C : AsRef < [u8] > > (path : P , contents : C) -> Result < () > { let path = path . as_ref () ; fs :: write (path , contents . as_ref ()) . with_context (| | format ! ("failed to write `{}`" , path . display ())) }
    };
}

write!();