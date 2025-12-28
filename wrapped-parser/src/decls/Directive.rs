macro_rules! deps {
    () => {
        Positioned!();
    };
}

macro_rules! Directive {
    () => {
        deps!();
        # [doc = " A GraphQL directive, such as `@deprecated(reason: \"Use the other field\")`."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#Directive)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct Directive { # [doc = " The name of the directive."] pub name : Positioned < Name > , # [doc = " The arguments to the directive."] pub arguments : Vec < (Positioned < Name > , Positioned < Value >) > , }
    };
}

Directive!()