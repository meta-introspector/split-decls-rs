// Generated macro for GraphQLError (enum)
macro_rules! DepcrateGraphQLError {
() => {
// Module: crate
// Provides: {"GraphQLError"}
// Dependencies: {}
# [doc = " An error that prevented query execution"] # [expect (missing_docs , reason = "self-explanatory")] # [derive (Clone , Debug , Display , Eq , From , PartialEq)] pub enum GraphQLError { ParseError (Spanning < ParseError >) , # [display ("{}" , _0 . iter () . format ("\n"))] ValidationError (Vec < RuleError >) , # [display ("No operation provided")] NoOperationProvided , # [display ("Multiple operations provided")] MultipleOperationsProvided , # [display ("Unknown operation name")] UnknownOperationName , # [display ("Operation is a subscription")] IsSubscription , # [display ("Operation is not a subscription")] NotSubscription , }
};
}
