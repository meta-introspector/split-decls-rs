macro_rules! deps {
    () => {
        Positioned!();
        EnumValueDefinition!();
    };
}

macro_rules! EnumType {
    () => {
        deps!();
        # [doc = " The definition of an enum."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#EnumType)."] # [derive (Debug , Clone)] pub struct EnumType { # [doc = " The possible values of the enum."] pub values : Vec < Positioned < EnumValueDefinition > > , }
    };
}

EnumType!()