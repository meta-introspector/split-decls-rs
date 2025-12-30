// Generated macro for is_serde_tag_or_content (function)
macro_rules! Depcrate_deis_serde_tag_or_content {
() => {
// Module: crate::de
// Provides: {"is_serde_tag_or_content"}
// Dependencies: {}
fn is_serde_tag_or_content < T > () -> bool { matches ! (core :: any :: type_name ::< T > () , "serde::__private::de::content::TagOrContent" | "serde::__private::de::content::TagOrContent<'_>") }
};
}
