macro_rules! deps {
    () => {
        Query!();
        Registry!();
        InputObject!();
        EmptyMutation!();
        Schema!();
        Object!();
        EmptySubscription!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] # [allow (clippy :: diverging_sub_expression)] mod tests { use super :: * ; use crate :: { parser :: parse_query , * } ; # [test] fn test_stringify () { let registry = Registry :: default () ; let doc = parse_query (r#"
            query Abc {
              a b c(a:1,b:2) {
                d e f
              }
            }
        "# ,) . unwrap () ; assert_eq ! (registry . stringify_exec_doc (& Default :: default () , & doc) . unwrap () , r#"query Abc { a b c(a: 1, b: 2) { d e f } }"#) ; let doc = parse_query (r#"
            query Abc($a:Int) {
              value(input:$a)
            }
        "# ,) . unwrap () ; assert_eq ! (registry . stringify_exec_doc (& Variables :: from_value (value ! ({ "a" : 10 , })) , & doc) . unwrap () , r#"query Abc($a: Int) { value(input: 10) }"#) ; } # [test] fn test_stringify_secret () { # [derive (InputObject)] # [graphql (internal)] struct MyInput { v1 : i32 , # [graphql (secret)] v2 : i32 , v3 : MyInput2 , } # [derive (InputObject)] # [graphql (internal)] struct MyInput2 { v4 : i32 , # [graphql (secret)] v5 : i32 , } struct Query ; # [Object (internal)] # [allow (unreachable_code , unused_variables)] impl Query { async fn value (& self , a : i32 , # [graphql (secret)] b : i32 , c : MyInput) -> i32 { todo ! () } } let schema = Schema :: new (Query , EmptyMutation , EmptySubscription) ; let registry = schema . registry () ; let s = registry . stringify_exec_doc (& Default :: default () , & parse_query (r#"
            {
                value(a: 10, b: 20, c: { v1: 1, v2: 2, v3: { v4: 4, v5: 5}})
            }
        "# ,) . unwrap () ,) . unwrap () ; assert_eq ! (s , r#"query { value(a: 10, b: "<secret>", c: {v1: 1, v2: "<secret>", v3: {v4: 4, v5: "<secret>"}}) }"#) ; } }
    };
}

tests!()