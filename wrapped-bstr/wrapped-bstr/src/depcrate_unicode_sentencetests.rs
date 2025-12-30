// Generated macro for tests (module)
macro_rules! Depcrate_unicode_sentencetests {
() => {
// Module: crate::unicode::sentence
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "std"))] mod tests { use alloc :: { vec , vec :: Vec } ; # [cfg (not (miri))] use ucd_parse :: SentenceBreakTest ; use crate :: ext_slice :: ByteSlice ; # [test] # [cfg (not (miri))] fn forward_ucd () { for (i , test) in ucdtests () . into_iter () . enumerate () { let given = test . sentences . concat () ; let got = sentences (given . as_bytes ()) ; assert_eq ! (test . sentences , got , "\n\nsentence forward break test {} failed:\n\
                 given:    {:?}\n\
                 expected: {:?}\n\
                 got:      {:?}\n" , i , given , strs_to_bstrs (& test . sentences) , strs_to_bstrs (& got) ,) ; } } # [test] fn forward_additional () { assert_eq ! (vec ! ["a.. " , "A"] , sentences (b"a.. A")) ; assert_eq ! (vec ! ["a.. a"] , sentences (b"a.. a")) ; assert_eq ! (vec ! ["a... " , "A"] , sentences (b"a... A")) ; assert_eq ! (vec ! ["a... a"] , sentences (b"a... a")) ; assert_eq ! (vec ! ["a...,..., a"] , sentences (b"a...,..., a")) ; } fn sentences (bytes : & [u8]) -> Vec < & str > { bytes . sentences () . collect () } # [cfg (not (miri))] fn strs_to_bstrs < S : AsRef < str > > (strs : & [S]) -> Vec < & [u8] > { strs . iter () . map (| s | s . as_ref () . as_bytes ()) . collect () } # [doc = " Return all of the UCD for sentence breaks."] # [cfg (not (miri))] fn ucdtests () -> Vec < SentenceBreakTest > { const TESTDATA : & str = include_str ! ("data/SentenceBreakTest.txt") ; let mut tests = vec ! [] ; for mut line in TESTDATA . lines () { line = line . trim () ; if line . starts_with ("#") || line . contains ("surrogate") { continue ; } tests . push (line . parse () . unwrap ()) ; } tests } }
};
}
