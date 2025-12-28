macro_rules! deps {
    () => {
        Cache!();
        MatchError!();
        Input!();
        DFA!();
        HalfMatch!();
    };
}

macro_rules! find_rev {
    () => {
        deps!();
        # [inline (never)] pub (crate) fn find_rev (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > ,) -> Result < Option < HalfMatch > , MatchError > { if input . is_done () { return Ok (None) ; } if input . get_earliest () { find_rev_imp (dfa , cache , input , true) } else { find_rev_imp (dfa , cache , input , false) } }
    };
}

find_rev!();