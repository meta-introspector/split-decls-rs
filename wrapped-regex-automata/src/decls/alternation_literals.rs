macro_rules! deps {
    () => {
        RegexInfo!();
        MatchKind!();
    };
}

macro_rules! alternation_literals {
    () => {
        deps!();
        # [doc = " Pull out an alternation of literals from the given sequence of HIR"] # [doc = " expressions."] # [doc = ""] # [doc = " There are numerous ways for this to fail. Generally, this only applies"] # [doc = " to regexes of the form 'foo|bar|baz|...|quux'. It can also fail if there"] # [doc = " are \"too few\" alternates, in which case, the regex engine is likely faster."] # [doc = ""] # [doc = " And currently, this only returns something when 'hirs.len() == 1'."] pub (crate) fn alternation_literals (info : & RegexInfo , hirs : & [& Hir] ,) -> Option < Vec < Vec < u8 > > > { use regex_syntax :: hir :: { HirKind , Literal } ; if ! cfg ! (feature = "perf-literal-multisubstring") { return None ; } if hirs . len () != 1 || ! info . props () [0] . look_set () . is_empty () || info . props () [0] . explicit_captures_len () > 0 || ! info . props () [0] . is_alternation_literal () || info . config () . get_match_kind () != MatchKind :: LeftmostFirst { return None ; } let hir = & hirs [0] ; let alts = match * hir . kind () { HirKind :: Alternation (ref alts) => alts , _ => return None , } ; let mut lits = vec ! [] ; for alt in alts { let mut lit = vec ! [] ; match * alt . kind () { HirKind :: Literal (Literal (ref bytes)) => { lit . extend_from_slice (bytes) } HirKind :: Concat (ref exprs) => { for e in exprs { match * e . kind () { HirKind :: Literal (Literal (ref bytes)) => { lit . extend_from_slice (bytes) ; } _ => unreachable ! ("expected literal, got {e:?}") , } } } _ => unreachable ! ("expected literal or concat, got {alt:?}") , } lits . push (lit) ; } if lits . len () < 3000 { debug ! ("skipping Aho-Corasick because there are too few literals") ; return None ; } Some (lits) }
    };
}

alternation_literals!()