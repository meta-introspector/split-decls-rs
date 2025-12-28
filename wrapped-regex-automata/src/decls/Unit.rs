macro_rules! deps {
    () => {
        DFA!();
        UnitKind!();
    };
}

macro_rules! Unit {
    () => {
        deps!();
        # [doc = " Unit represents a single unit of haystack for DFA based regex engines."] # [doc = ""] # [doc = " It is not expected for consumers of this crate to need to use this type"] # [doc = " unless they are implementing their own DFA. And even then, it's not"] # [doc = " required: implementors may use other techniques to handle haystack units."] # [doc = ""] # [doc = " Typically, a single unit of haystack for a DFA would be a single byte."] # [doc = " However, for the DFAs in this crate, matches are delayed by a single byte"] # [doc = " in order to handle look-ahead assertions (`\\b`, `$` and `\\z`). Thus, once"] # [doc = " we have consumed the haystack, we must run the DFA through one additional"] # [doc = " transition using a unit that indicates the haystack has ended."] # [doc = ""] # [doc = " There is no way to represent a sentinel with a `u8` since all possible"] # [doc = " values *may* be valid haystack units to a DFA, therefore this type"] # [doc = " explicitly adds room for a sentinel value."] # [doc = ""] # [doc = " The sentinel EOI value is always its own equivalence class and is"] # [doc = " ultimately represented by adding 1 to the maximum equivalence class value."] # [doc = " So for example, the regex `^[a-z]+$` might be split into the following"] # [doc = " equivalence classes:"] # [doc = ""] # [doc = " ```text"] # [doc = " 0 => [\\x00-`]"] # [doc = " 1 => [a-z]"] # [doc = " 2 => [{-\\xFF]"] # [doc = " 3 => [EOI]"] # [doc = " ```"] # [doc = ""] # [doc = " Where EOI is the special sentinel value that is always in its own"] # [doc = " singleton equivalence class."] # [derive (Clone , Copy , Eq , PartialEq , PartialOrd , Ord)] pub struct Unit (UnitKind) ;
    };
}

Unit!();