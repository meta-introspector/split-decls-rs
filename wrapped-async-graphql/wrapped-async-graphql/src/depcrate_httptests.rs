// Generated macro for tests (module)
macro_rules! Depcrate_httptests {
() => {
// Module: crate::http
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: collections :: HashMap ; use async_graphql_value :: Extensions ; use super :: * ; use crate :: { Variables , value } ; # [test] fn test_parse_query_string () { let request = parse_query_string ("variables=%7B%7D&extensions=%7B%22persistedQuery%22%3A%7B%22sha256Hash%22%3A%22cde5de0a350a19c59f8ddcd9646e5f260b2a7d5649ff6be8e63e9462934542c3%22%2C%22version%22%3A1%7D%7D") . unwrap () ; assert_eq ! (request . query . as_str () , "") ; assert_eq ! (request . variables , Variables :: default ()) ; assert_eq ! (request . extensions , { let mut extensions = HashMap :: new () ; extensions . insert ("persistedQuery" . to_string () , value ! ({ "sha256Hash" : "cde5de0a350a19c59f8ddcd9646e5f260b2a7d5649ff6be8e63e9462934542c3" , "version" : 1 , })) ; Extensions (extensions) }) ; let request = parse_query_string ("query={a}&variables=%7B%22a%22%3A10%7D") . unwrap () ; assert_eq ! (request . query . as_str () , "{a}") ; assert_eq ! (request . variables , Variables :: from_value (value ! ({ "a" : 10 }))) ; } }
};
}
