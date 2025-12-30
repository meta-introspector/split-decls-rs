// Generated macro for execute (function)
macro_rules! Depcrateexecute {
() => {
// Module: crate
// Provides: {"execute"}
// Dependencies: {}
# [doc = " Execute a query in a provided schema"] pub async fn execute < 'a , S , QueryT , MutationT , SubscriptionT > (document_source : & 'a str , operation_name : Option < & str > , root_node : & 'a RootNode < QueryT , MutationT , SubscriptionT , S > , variables : & Variables < S > , context : & QueryT :: Context ,) -> Result < (Value < S > , Vec < ExecutionError < S > >) , GraphQLError > where QueryT : GraphQLTypeAsync < S > , QueryT :: TypeInfo : Sync , QueryT :: Context : Sync , MutationT : GraphQLTypeAsync < S , Context = QueryT :: Context > , MutationT :: TypeInfo : Sync , SubscriptionT : GraphQLType < S , Context = QueryT :: Context > + Sync , SubscriptionT :: TypeInfo : Sync , S : ScalarValue + Send + Sync , { let document = parse_document_source (document_source , & root_node . schema) ? ; { let mut ctx = ValidatorContext :: new (& root_node . schema , & document) ; visit_all_rules (& mut ctx , & document) ; if root_node . introspection_disabled { visit_rule (& mut MultiVisitorNil . with (rules :: disable_introspection :: factory ()) , & mut ctx , & document ,) ; } let errors = ctx . into_errors () ; if ! errors . is_empty () { return Err (errors . into ()) ; } } let operation = get_operation (& document , operation_name) ? ; { let errors = validate_input_values (variables , operation , & root_node . schema) ; if ! errors . is_empty () { return Err (errors . into ()) ; } } executor :: execute_validated_query_async (& document , operation , root_node , variables , context) . await }
};
}
