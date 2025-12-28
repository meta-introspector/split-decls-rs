macro_rules! deps {
    () => {
        ResolvedRule!();
    };
}

macro_rules! Matcher {
    () => {
        deps!();
        # [doc = " Checks if our search pattern matches a particular node of the AST."] struct Matcher < 'db , 'sema > { sema : & 'sema Semantics < 'db , ide_db :: RootDatabase > , # [doc = " If any placeholders come from anywhere outside of this range, then the match will be"] # [doc = " rejected."] restrict_range : Option < FileRange > , rule : & 'sema ResolvedRule < 'db > , }
    };
}

Matcher!()