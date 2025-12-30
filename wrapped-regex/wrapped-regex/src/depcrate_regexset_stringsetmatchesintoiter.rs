// Generated macro for SetMatchesIntoIter (struct)
macro_rules! Depcrate_regexset_stringSetMatchesIntoIter {
() => {
// Module: crate::regexset::string
// Provides: {"SetMatchesIntoIter"}
// Dependencies: {}
# [doc = " An owned iterator over the set of matches from a regex set."] # [doc = ""] # [doc = " This will always produces matches in ascending order of index, where the"] # [doc = " index corresponds to the index of the regex that matched with respect to"] # [doc = " its position when initially building the set."] # [doc = ""] # [doc = " This iterator is created by calling `SetMatches::into_iter` via the"] # [doc = " `IntoIterator` trait. This is automatically done in `for` loops."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use regex::RegexSet;"] # [doc = ""] # [doc = " let set = RegexSet::new(["] # [doc = "     r\"[0-9]\","] # [doc = "     r\"[a-z]\","] # [doc = "     r\"[A-Z]\","] # [doc = "     r\"\\p{Greek}\","] # [doc = " ]).unwrap();"] # [doc = " let hay = \"βa1\";"] # [doc = " let mut matches = vec![];"] # [doc = " for index in set.matches(hay) {"] # [doc = "     matches.push(index);"] # [doc = " }"] # [doc = " assert_eq!(matches, vec![0, 1, 3]);"] # [doc = " ```"] # [derive (Debug)] pub struct SetMatchesIntoIter { patset : PatternSet , it : core :: ops :: Range < usize > , }
};
}
