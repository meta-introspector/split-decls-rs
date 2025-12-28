macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! parse_many {
    () => {
        deps!();
        # [doc = " A convenience routine for parsing many patterns into HIR value with the"] # [doc = " default configuration."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This shows how to parse many patterns into an corresponding HIR values:"] # [doc = ""] # [doc = " ```"] # [doc = " use {"] # [doc = "     regex_automata::util::syntax,"] # [doc = "     regex_syntax::hir::Properties,"] # [doc = " };"] # [doc = ""] # [doc = " let hirs = syntax::parse_many(&["] # [doc = "     r\"([a-z]+)|([0-9]+)\","] # [doc = "     r\"foo(A-Z]+)bar\","] # [doc = " ])?;"] # [doc = " let props = Properties::union(hirs.iter().map(|h| h.properties()));"] # [doc = " assert_eq!(Some(1), props.static_explicit_captures_len());"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn parse_many < P : AsRef < str > > (patterns : & [P]) -> Result < Vec < Hir > , Error > { parse_many_with (patterns , & Config :: default ()) }
    };
}

parse_many!();