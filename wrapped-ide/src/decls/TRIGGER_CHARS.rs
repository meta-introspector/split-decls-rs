macro_rules! TRIGGER_CHARS {
    () => {
        pub (crate) const TRIGGER_CHARS : & [char] = & ['.' , '=' , '<' , '>' , '{' , '(' , '|' , '+'] ;
    };
}

TRIGGER_CHARS!()