macro_rules! open {
    () => {
        # [doc = " Opens an existing file."] pub fn open < P : AsRef < Path > > (path : P) -> Result < File > { let path = path . as_ref () ; File :: open (path) . with_context (| | format ! ("failed to open file `{}`" , path . display ())) }
    };
}

open!()