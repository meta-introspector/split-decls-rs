macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! parse_many_with {
    () => {
        deps!();
        # [doc = " A convenience routine for parsing many patterns into HIR values using a"] # [doc = " `Config`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This shows how to parse many patterns into an corresponding HIR values"] # [doc = " with a non-default configuration:"] # [doc = ""] # [doc = " ```"] # [doc = " use {"] # [doc = "     regex_automata::util::syntax,"] # [doc = "     regex_syntax::hir::Properties,"] # [doc = " };"] # [doc = ""] # [doc = " let patterns = &["] # [doc = "     r\"([a-z]+)|([0-9]+)\","] # [doc = "     r\"\\W\","] # [doc = "     r\"foo(A-Z]+)bar\","] # [doc = " ];"] # [doc = " let config = syntax::Config::new().unicode(false).utf8(false);"] # [doc = " let hirs = syntax::parse_many_with(patterns, &config)?;"] # [doc = " let props = Properties::union(hirs.iter().map(|h| h.properties()));"] # [doc = " assert!(!props.is_utf8());"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn parse_many_with < P : AsRef < str > > (patterns : & [P] , config : & Config ,) -> Result < Vec < Hir > , Error > { let mut builder = ParserBuilder :: new () ; config . apply (& mut builder) ; let mut hirs = vec ! [] ; for p in patterns . iter () { hirs . push (builder . build () . parse (p . as_ref ()) ?) ; } Ok (hirs) }
    };
}

parse_many_with!();