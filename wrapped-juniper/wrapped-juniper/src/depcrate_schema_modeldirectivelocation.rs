// Generated macro for DirectiveLocation (enum)
macro_rules! Depcrate_schema_modelDirectiveLocation {
() => {
// Module: crate::schema::model
// Provides: {"DirectiveLocation"}
// Dependencies: {}
# [derive (Clone , Debug , Display , Eq , GraphQLEnum , PartialEq)] # [graphql (name = "__DirectiveLocation" , internal)] pub enum DirectiveLocation { # [display ("query")] Query , # [display ("mutation")] Mutation , # [display ("subscription")] Subscription , # [display ("field")] Field , # [display ("fragment definition")] FragmentDefinition , # [display ("fragment spread")] FragmentSpread , # [display ("inline fragment")] InlineFragment , # [display ("variable definition")] VariableDefinition , # [display ("schema")] Schema , # [display ("scalar")] Scalar , # [display ("object")] Object , # [display ("field definition")] FieldDefinition , # [display ("argument definition")] ArgumentDefinition , # [display ("interface")] Interface , # [display ("union")] Union , # [display ("enum")] Enum , # [display ("enum value")] EnumValue , # [display ("input object")] InputObject , # [display ("input field definition")] InputFieldDefinition , }
};
}
