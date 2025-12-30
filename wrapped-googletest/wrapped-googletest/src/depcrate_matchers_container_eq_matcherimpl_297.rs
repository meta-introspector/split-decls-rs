// Generated macro for impl_297 (impl)
macro_rules! Depcrate_matchers_container_eq_matcherimpl_297 {
() => {
// Module: crate::matchers::container_eq_matcher
// Provides: {"impl_297"}
// Dependencies: {}
impl < ExpectedElementT , ExpectedContainerT > ContainerEqMatcher < ExpectedContainerT > where for < 'a > & 'a ExpectedContainerT : IntoIterator < Item = & 'a ExpectedElementT > , { fn get_missing_items < ActualElementT , ActualContainerT > (& self , actual : ActualContainerT ,) -> Vec < & '_ ExpectedElementT > where ActualElementT : for < 'a > PartialEq < & 'a ExpectedElementT > + Copy , ActualContainerT : for < 'a > PartialEq < & 'a ExpectedContainerT > + Copy , ActualContainerT : IntoIterator < Item = ActualElementT > , { self . expected . into_iter () . filter (| i | ! actual . into_iter () . any (| j | j == * i)) . collect () } fn get_unexpected_items < ActualElementT , ActualContainerT > (& self , actual : ActualContainerT ,) -> Vec < ActualElementT > where ActualElementT : for < 'a > PartialEq < & 'a ExpectedElementT > + Copy , ActualContainerT : for < 'a > PartialEq < & 'a ExpectedContainerT > + Copy , ActualContainerT : IntoIterator < Item = ActualElementT > , { actual . into_iter () . filter (| i | ! self . expected . into_iter () . any (| j | i == & j)) . collect () } }
};
}
