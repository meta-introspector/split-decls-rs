// Generated macro for tests (module)
macro_rules! Depcrate_iterators_pairtests {
() => {
// Module: crate::iterators::pair
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: macros :: tests :: * ; use crate :: parser :: Parser ; # [test] # [cfg (feature = "pretty-print")] fn test_pretty_print () { let pair = AbcParser :: parse (Rule :: a , "abcde") . unwrap () . next () . unwrap () ; let expected = r#"{
  "pos": [
    0,
    3
  ],
  "rule": "a",
  "inner": {
    "pos": [
      1,
      2
    ],
    "pairs": [
      {
        "pos": [
          1,
          2
        ],
        "rule": "b",
        "inner": "b"
      }
    ]
  }
}"# ; assert_eq ! (expected , pair . to_json ()) ; } # [test] fn pair_into_inner () { let pair = AbcParser :: parse (Rule :: a , "abcde") . unwrap () . next () . unwrap () ; let pairs = pair . into_inner () ; assert_eq ! (2 , pairs . tokens () . count ()) ; } # [test] fn get_input_of_pair () { let input = "abcde" ; let pair = AbcParser :: parse (Rule :: a , input) . unwrap () . next () . unwrap () ; assert_eq ! (input , pair . get_input ()) ; } }
};
}
