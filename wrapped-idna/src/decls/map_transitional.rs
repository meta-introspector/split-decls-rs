macro_rules! map_transitional {
    () => {
        # [doc = " Performs preprocessing equivalent to UTS 46 transitional processing"] # [doc = " if `transitional` is `true`. If `transitional` is `false`, merely"] # [doc = " lets the input pass through as-is (for call site convenience)."] # [doc = ""] # [doc = " The output of this function is to be passed to [`Uts46::process`]."] fn map_transitional (domain : & str , transitional : bool) -> Cow < '_ , str > { if ! transitional { return Cow :: Borrowed (domain) ; } let mut chars = domain . chars () ; loop { let prev = chars . clone () ; if let Some (c) = chars . next () { match c { 'ß' | 'ẞ' | 'ς' | '\u{200C}' | '\u{200D}' => { let mut s = String :: with_capacity (domain . len ()) ; let tail = prev . as_str () ; let head = & domain [.. domain . len () - tail . len ()] ; s . push_str (head) ; for c in tail . chars () { match c { 'ß' | 'ẞ' => { s . push_str ("ss") ; } 'ς' => { s . push ('σ') ; } '\u{200C}' | '\u{200D}' => { } _ => { s . push (c) ; } } } return Cow :: Owned (s) ; } _ => { } } } else { break ; } } Cow :: Borrowed (domain) }
    };
}

map_transitional!()