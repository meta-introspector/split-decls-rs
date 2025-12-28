macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! parse_with {
    () => {
        deps!();
        # [doc = " A convenience routine for parsing a pattern into an HIR value using a"] # [doc = " `Config`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This shows how to parse a pattern into an HIR value with a non-default"] # [doc = " configuration:"] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::util::syntax;"] # [doc = ""] # [doc = " let hir = syntax::parse_with("] # [doc = "     r\"^[a-z]+$\","] # [doc = "     &syntax::Config::new().multi_line(true).crlf(true),"] # [doc = " )?;"] # [doc = " assert!(hir.properties().look_set().contains_anchor_crlf());"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn parse_with (pattern : & str , config : & Config) -> Result < Hir , Error > { let mut builder = ParserBuilder :: new () ; config . apply (& mut builder) ; builder . build () . parse (pattern) }
    };
}

parse_with!();