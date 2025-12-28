macro_rules! deps {
    () => {
        Reject!();
        Cursor!();
    };
}

macro_rules! word_break {
    () => {
        deps!();
        fn word_break (input : Cursor) -> Result < Cursor , Reject > { match input . chars () . next () { Some (ch) if is_ident_continue (ch) => Err (Reject) , Some (_) | None => Ok (input) , } }
    };
}

word_break!()