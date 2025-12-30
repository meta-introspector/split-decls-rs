// Generated macro for is_serde_content (function)
macro_rules! Depcrate_deis_serde_content {
() => {
// Module: crate::de
// Provides: {"is_serde_content"}
// Dependencies: {}
fn is_serde_content < T > () -> bool { matches ! (core :: any :: type_name ::< T > () , "serde::__private::de::content::Content" | "serde::__private::de::content::Content<'_>") }
};
}
