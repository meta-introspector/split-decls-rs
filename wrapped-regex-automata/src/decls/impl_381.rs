macro_rules! deps {
    () => {
        Memchr3!();
        Choice!();
        Strategy!();
        MatchKind!();
        RegexInfo!();
        Memmem!();
        Teddy!();
        ByteSet!();
        Pre!();
        AhoCorasick!();
        Memchr!();
        Memchr2!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        impl Pre < () > { # [doc = " Given a sequence of prefixes, attempt to return a full `Strategy` using"] # [doc = " just the prefixes."] # [doc = ""] # [doc = " Basically, this occurs when the prefixes given not just prefixes,"] # [doc = " but an enumeration of the entire language matched by the regular"] # [doc = " expression."] # [doc = ""] # [doc = " A number of other conditions need to be true too. For example, there"] # [doc = " can be only one pattern, the number of explicit capture groups is 0, no"] # [doc = " look-around assertions and so on."] # [doc = ""] # [doc = " Note that this ignores `Config::get_auto_prefilter` because if this"] # [doc = " returns something, then it isn't a prefilter but a matcher itself."] # [doc = " Therefore, it shouldn't suffer from the problems typical to prefilters"] # [doc = " (such as a high false positive rate)."] fn from_prefixes (info : & RegexInfo , prefixes : & literal :: Seq ,) -> Option < Arc < dyn Strategy > > { let kind = info . config () . get_match_kind () ; if ! prefixes . is_exact () { return None ; } if info . pattern_len () != 1 { return None ; } if info . props () [0] . explicit_captures_len () != 0 { return None ; } if ! info . props () [0] . look_set () . is_empty () { return None ; } if kind != MatchKind :: LeftmostFirst { return None ; } let prefixes = prefixes . literals () . unwrap () ; debug ! ("trying to bypass regex engine by creating \
             prefilter from {} literals: {:?}" , prefixes . len () , prefixes ,) ; let choice = match prefilter :: Choice :: new (kind , prefixes) { Some (choice) => choice , None => { debug ! ("regex bypass failed because no prefilter could be built") ; return None ; } } ; let strat : Arc < dyn Strategy > = match choice { prefilter :: Choice :: Memchr (pre) => Pre :: new (pre) , prefilter :: Choice :: Memchr2 (pre) => Pre :: new (pre) , prefilter :: Choice :: Memchr3 (pre) => Pre :: new (pre) , prefilter :: Choice :: Memmem (pre) => Pre :: new (pre) , prefilter :: Choice :: Teddy (pre) => Pre :: new (pre) , prefilter :: Choice :: ByteSet (pre) => Pre :: new (pre) , prefilter :: Choice :: AhoCorasick (pre) => Pre :: new (pre) , } ; Some (strat) } # [doc = " Attempts to extract an alternation of literals, and if it's deemed"] # [doc = " worth doing, returns an Aho-Corasick prefilter as a strategy."] # [doc = ""] # [doc = " And currently, this only returns something when 'hirs.len() == 1'. This"] # [doc = " could in theory do something if there are multiple HIRs where all of"] # [doc = " them are alternation of literals, but I haven't had the time to go down"] # [doc = " that path yet."] fn from_alternation_literals (info : & RegexInfo , hirs : & [& Hir] ,) -> Option < Arc < dyn Strategy > > { use crate :: util :: prefilter :: AhoCorasick ; let lits = crate :: meta :: literal :: alternation_literals (info , hirs) ? ; let ac = AhoCorasick :: new (MatchKind :: LeftmostFirst , & lits) ? ; Some (Pre :: new (ac)) } }
    };
}

impl_381!();