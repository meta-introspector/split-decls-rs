// Generated macro for actual_is_missing (function)
macro_rules! Depcrate_matchers_superset_of_matcheractual_is_missing {
() => {
// Module: crate::matchers::superset_of_matcher
// Provides: {"actual_is_missing"}
// Dependencies: {}
fn actual_is_missing < ElementT : PartialEq , ActualT > (actual : ActualT , needle : & ElementT) -> bool where ActualT : IntoIterator < Item = ElementT > , { ! actual . into_iter () . any (| item | & item == needle) }
};
}
