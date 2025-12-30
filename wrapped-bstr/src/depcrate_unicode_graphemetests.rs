// Generated macro for tests (module)
macro_rules! Depcrate_unicode_graphemetests {
() => {
// Module: crate::unicode::grapheme
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "std"))] mod tests { use alloc :: { string :: { String , ToString } , vec , vec :: Vec , } ; # [cfg (not (miri))] use ucd_parse :: GraphemeClusterBreakTest ; use crate :: tests :: LOSSY_TESTS ; use super :: * ; # [test] # [cfg (not (miri))] fn forward_ucd () { for (i , test) in ucdtests () . into_iter () . enumerate () { let given = test . grapheme_clusters . concat () ; let got : Vec < String > = Graphemes :: new (given . as_bytes ()) . map (| cluster | cluster . to_string ()) . collect () ; assert_eq ! (test . grapheme_clusters , got , "\ngrapheme forward break test {} failed:\n\
                 given:    {:?}\n\
                 expected: {:?}\n\
                 got:      {:?}\n" , i , uniescape (& given) , uniescape_vec (& test . grapheme_clusters) , uniescape_vec (& got) ,) ; } } # [test] # [cfg (not (miri))] fn reverse_ucd () { for (i , test) in ucdtests () . into_iter () . enumerate () { let given = test . grapheme_clusters . concat () ; let mut got : Vec < String > = Graphemes :: new (given . as_bytes ()) . rev () . map (| cluster | cluster . to_string ()) . collect () ; got . reverse () ; assert_eq ! (test . grapheme_clusters , got , "\n\ngrapheme reverse break test {} failed:\n\
                 given:    {:?}\n\
                 expected: {:?}\n\
                 got:      {:?}\n" , i , uniescape (& given) , uniescape_vec (& test . grapheme_clusters) , uniescape_vec (& got) ,) ; } } # [test] fn forward_lossy () { for & (expected , input) in LOSSY_TESTS { let got = Graphemes :: new (input . as_bytes ()) . collect :: < String > () ; assert_eq ! (expected , got) ; } } # [test] fn reverse_lossy () { for & (expected , input) in LOSSY_TESTS { let expected : String = expected . chars () . rev () . collect () ; let got = Graphemes :: new (input . as_bytes ()) . rev () . collect :: < String > () ; assert_eq ! (expected , got) ; } } # [cfg (not (miri))] fn uniescape (s : & str) -> String { s . chars () . flat_map (| c | c . escape_unicode ()) . collect :: < String > () } # [cfg (not (miri))] fn uniescape_vec (strs : & [String]) -> Vec < String > { strs . iter () . map (| s | uniescape (s)) . collect () } # [doc = " Return all of the UCD for grapheme breaks."] # [cfg (not (miri))] fn ucdtests () -> Vec < GraphemeClusterBreakTest > { const TESTDATA : & str = include_str ! ("data/GraphemeBreakTest.txt") ; let mut tests = vec ! [] ; for mut line in TESTDATA . lines () { line = line . trim () ; if line . starts_with ("#") || line . contains ("surrogate") { continue ; } tests . push (line . parse () . unwrap ()) ; } tests } }
};
}
