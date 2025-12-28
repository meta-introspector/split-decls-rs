macro_rules! create {
    () => {
        # [doc = " Creates a new file."] pub fn create < P : AsRef < Path > > (path : P) -> Result < File > { let path = path . as_ref () ; File :: create (path) . with_context (| | format ! ("failed to create file `{}`" , path . display ())) }
    };
}

create!()