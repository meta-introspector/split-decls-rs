macro_rules! deps {
    () => {
        Options!();
        Kind!();
        Repository!();
        Reference!();
        Path!();
        Note!();
        Error!();
    };
}

macro_rules! Delegate {
    () => {
        deps!();
        struct Delegate < 'repo > { refs : [Option < gix_ref :: Reference > ; 2] , objs : [Option < HashSet < ObjectId > > ; 2] , # [doc = " Path specified like `@:<path>` or `:<path>` for later use when looking up specs."] # [doc = " Note that it terminates spec parsing, so it's either `0` or `1`, never both."] paths : [Option < (BString , gix_object :: tree :: EntryMode) > ; 2] , # [doc = " The originally encountered ambiguous objects for potential later use in errors."] ambiguous_objects : [Option < HashSet < ObjectId > > ; 2] , idx : usize , kind : Option < gix_revision :: spec :: Kind > , opts : Options , err : Vec < Error > , # [doc = " The ambiguous prefix obtained during a call to `disambiguate_prefix()`."] prefix : [Option < gix_hash :: Prefix > ; 2] , # [doc = " If true, we didn't try to do any other transformation which might have helped with disambiguation."] last_call_was_disambiguate_prefix : [bool ; 2] , repo : & 'repo Repository , }
    };
}

Delegate!()