macro_rules! deps {
    () => {
        TypeSystemDefinition!();
    };
}

macro_rules! ServiceDocument {
    () => {
        deps!();
        # [doc = " A GraphQL file or request string defining a GraphQL service."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#Document)."] # [derive (Debug , Clone)] pub struct ServiceDocument { # [doc = " The definitions of this document."] pub definitions : Vec < TypeSystemDefinition > , }
    };
}

ServiceDocument!()