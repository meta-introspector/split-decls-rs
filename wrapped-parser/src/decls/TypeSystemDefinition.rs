macro_rules! deps {
    () => {
        TypeDefinition!();
        Directive!();
        Type!();
        Positioned!();
        DirectiveDefinition!();
        SchemaDefinition!();
    };
}

macro_rules! TypeSystemDefinition {
    () => {
        deps!();
        # [doc = " A definition concerning the type system of a GraphQL service."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#TypeSystemDefinition). This enum also covers"] # [doc = " [extensions](https://spec.graphql.org/October2021/#TypeSystemExtension)."] # [derive (Debug , Clone)] pub enum TypeSystemDefinition { # [doc = " The definition of the schema of the service."] Schema (Positioned < SchemaDefinition >) , # [doc = " The definition of a type in the service."] Type (Positioned < TypeDefinition >) , # [doc = " The definition of a directive in the service."] Directive (Positioned < DirectiveDefinition >) , }
    };
}

TypeSystemDefinition!()