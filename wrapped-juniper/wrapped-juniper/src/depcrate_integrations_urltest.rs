// Generated macro for test (module)
macro_rules! Depcrate_integrations_urltest {
() => {
// Module: crate::integrations::url
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use url :: Url ; use crate :: { InputValue , graphql } ; # [test] fn url_from_input () { let raw = "https://example.net/" ; let input : InputValue = graphql :: input_value ! ((raw)) ; let parsed : Url = crate :: FromInputValue :: from_input_value (& input) . unwrap () ; let url = Url :: parse (raw) . unwrap () ; assert_eq ! (parsed , url) ; } }
};
}
