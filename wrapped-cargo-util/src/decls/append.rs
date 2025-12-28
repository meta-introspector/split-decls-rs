macro_rules! append {
    () => {
        # [doc = " Equivalent to [`write()`], but appends to the end instead of replacing the"] # [doc = " contents."] pub fn append (path : & Path , contents : & [u8]) -> Result < () > { (| | -> Result < () > { let mut f = OpenOptions :: new () . write (true) . append (true) . create (true) . open (path) ? ; f . write_all (contents) ? ; Ok (()) }) () . with_context (| | format ! ("failed to write `{}`" , path . display ())) ? ; Ok (()) }
    };
}

append!();