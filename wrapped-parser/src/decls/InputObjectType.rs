macro_rules! deps {
    () => {
        Positioned!();
        InputValueDefinition!();
    };
}

macro_rules! InputObjectType {
    () => {
        deps!();
        # [doc = " The definition of an input object."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#InputObjectType)."] # [derive (Debug , Clone)] pub struct InputObjectType { # [doc = " The fields of the input object."] pub fields : Vec < Positioned < InputValueDefinition > > , }
    };
}

InputObjectType!()