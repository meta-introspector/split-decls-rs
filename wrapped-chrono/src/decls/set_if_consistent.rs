macro_rules! deps {
    () => {
        ParseResult!();
    };
}

macro_rules! set_if_consistent {
    () => {
        deps!();
        # [doc = " Checks if `old` is either empty or has the same value as `new` (i.e. \"consistent\"),"] # [doc = " and if it is empty, set `old` to `new` as well."] # [inline] fn set_if_consistent < T : PartialEq > (old : & mut Option < T > , new : T) -> ParseResult < () > { match old { Some (old) if * old != new => Err (IMPOSSIBLE) , _ => { * old = Some (new) ; Ok (()) } } }
    };
}

set_if_consistent!();