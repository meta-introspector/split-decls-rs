// Generated macro for accepts_enum_literal (function)
macro_rules! Depcrate_executor_tests_enumsaccepts_enum_literal {
() => {
// Module: crate::executor_tests::enums
// Provides: {"accepts_enum_literal"}
// Dependencies: {}
# [tokio :: test] async fn accepts_enum_literal () { run_query ("{ toString(color: RED) }" , | result | { assert_eq ! (result . get_field_value ("toString") , Some (& graphql :: value ! ("Color::Red")) ,) ; }) . await ; }
};
}
