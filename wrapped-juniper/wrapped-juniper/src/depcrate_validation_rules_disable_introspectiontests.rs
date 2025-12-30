// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_disable_introspectiontests {
() => {
// Module: crate::validation::rules::disable_introspection
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn allows_regular_fields () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
            query {
                user {
                    name
                    ... on User {
                        email
                    }
                    alias: email
                    ... {
                        typeless
                    }
                    friends {
                        name
                    }
                }
            }
            "# ,) ; } # [test] fn allows_typename_field () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
            query {
                __typename
                user {
                    __typename
                    ... on User {
                        __typename
                    }
                    ... {
                        __typename
                    }
                    friends {
                        __typename
                    }
                }
            }
            "# ,) ; } # [test] fn forbids_query_schema () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
            query {
                __schema {
                    queryType {
                       name
                    }
                }
            }
            "# , & [RuleError :: new (& error_message ("__schema") , & [SourcePosition :: new (37 , 2 , 16)] ,)] ,) ; } # [test] fn forbids_query_type () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
            query {
                __type(
                   name: "Query"
                ) {
                   name
                }
            }
            "# , & [RuleError :: new (& error_message ("__type") , & [SourcePosition :: new (37 , 2 , 16)] ,)] ,) ; } # [test] fn forbids_field_type () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
            query {
                user {
                    name
                    ... on User {
                        email
                    }
                    alias: email
                    ... {
                        typeless
                    }
                    friends {
                        name
                    }
                    __type
                }
            }
            "# , & [RuleError :: new (& error_message ("__type") , & [SourcePosition :: new (370 , 14 , 20)] ,)] ,) ; } # [test] fn forbids_field_schema () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
            query {
                user {
                    name
                    ... on User {
                        email
                    }
                    alias: email
                    ... {
                        typeless
                    }
                    friends {
                        name
                    }
                    __schema
                }
            }
            "# , & [RuleError :: new (& error_message ("__schema") , & [SourcePosition :: new (370 , 14 , 20)] ,)] ,) ; } }
};
}
