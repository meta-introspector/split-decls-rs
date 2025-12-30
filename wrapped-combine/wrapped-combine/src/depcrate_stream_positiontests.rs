// Generated macro for tests (module)
macro_rules! Depcrate_stream_positiontests {
() => {
// Module: crate::stream::position
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (feature = "std" , test))] mod tests { use crate :: Parser ; use super :: * ; # [test] fn test_positioner () { let input = ["a" . to_string () , "b" . to_string ()] ; let mut parser = crate :: any () ; let result = parser . parse (Stream :: new (& input [..])) ; assert_eq ! (result , Ok (("a" . to_string () , Stream :: with_positioner (& ["b" . to_string ()] [..] , IndexPositioner :: new_with_position (1))))) ; } # [test] fn test_range_positioner () { let input = ["a" . to_string () , "b" . to_string () , "c" . to_string ()] ; let mut parser = crate :: parser :: range :: take (2) ; let result = parser . parse (Stream :: new (& input [..])) ; assert_eq ! (result , Ok ((& ["a" . to_string () , "b" . to_string ()] [..] , Stream :: with_positioner (& ["c" . to_string ()] [..] , IndexPositioner :: new_with_position (2))))) ; } }
};
}
