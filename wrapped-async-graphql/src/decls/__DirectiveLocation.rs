macro_rules! deps {
    () => {
        Directive!();
    };
}

macro_rules! __DirectiveLocation {
    () => {
        deps!();
        # [doc = " A Directive can be adjacent to many parts of the GraphQL language, a"] # [doc = " __DirectiveLocation describes one such possible adjacencies."] # [derive (Debug , Enum , Copy , Clone , Eq , PartialEq)] # [graphql (internal , name = "__DirectiveLocation")] # [allow (non_camel_case_types)] pub enum __DirectiveLocation { # [doc = " Location adjacent to a query operation."] QUERY , # [doc = " Location adjacent to a mutation operation."] MUTATION , # [doc = " Location adjacent to a subscription operation."] SUBSCRIPTION , # [doc = " Location adjacent to a field."] FIELD , # [doc = " Location adjacent to a fragment definition."] FRAGMENT_DEFINITION , # [doc = " Location adjacent to a fragment spread."] FRAGMENT_SPREAD , # [doc = " Location adjacent to an inline fragment."] INLINE_FRAGMENT , # [doc = " Location adjacent to a variable definition."] VARIABLE_DEFINITION , # [doc = " Location adjacent to a schema definition."] SCHEMA , # [doc = " Location adjacent to a scalar definition."] SCALAR , # [doc = " Location adjacent to an object type definition."] OBJECT , # [doc = " Location adjacent to a field definition."] FIELD_DEFINITION , # [doc = " Location adjacent to an argument definition."] ARGUMENT_DEFINITION , # [doc = " Location adjacent to an interface definition."] INTERFACE , # [doc = " Location adjacent to a union definition."] UNION , # [doc = " Location adjacent to an enum definition."] ENUM , # [doc = " Location adjacent to an enum value definition."] ENUM_VALUE , # [doc = " Location adjacent to an input object type definition."] INPUT_OBJECT , # [doc = " Location adjacent to an input object field definition."] INPUT_FIELD_DEFINITION , }
    };
}

__DirectiveLocation!()