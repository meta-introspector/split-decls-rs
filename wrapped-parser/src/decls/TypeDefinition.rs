macro_rules! deps {
    () => {
        Positioned!();
        ConstDirective!();
        TypeKind!();
    };
}

macro_rules! TypeDefinition {
    () => {
        deps!();
        # [doc = " The definition of a type in a GraphQL service."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#TypeDefinition). This also covers"] # [doc = " [extensions](https://spec.graphql.org/October2021/#TypeExtension)."] # [derive (Debug , Clone)] pub struct TypeDefinition { # [doc = " Whether the type is an extension of another type."] pub extend : bool , # [doc = " The description of the type, if present. This is never present on an"] # [doc = " extension type."] pub description : Option < Positioned < String > > , # [doc = " The name of the type."] pub name : Positioned < Name > , # [doc = " The directives of type definition."] pub directives : Vec < Positioned < ConstDirective > > , # [doc = " Which kind of type is being defined; scalar, object, enum, etc."] pub kind : TypeKind , }
    };
}

TypeDefinition!();