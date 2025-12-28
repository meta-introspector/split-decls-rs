macro_rules! deps {
    () => {
        BaseType!();
    };
}

macro_rules! Type {
    () => {
        deps!();
        # [doc = " A GraphQL type, for example `String` or `[String!]!`."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#Type)."] # [derive (Debug , PartialEq , Eq , Clone , Serialize , Deserialize)] pub struct Type { # [doc = " The base type."] pub base : BaseType , # [doc = " Whether the type is nullable."] pub nullable : bool , }
    };
}

Type!();