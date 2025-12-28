macro_rules! deps {
    () => {
        Positioned!();
        FieldDefinition!();
    };
}

macro_rules! InterfaceType {
    () => {
        deps!();
        # [doc = " The definition of an interface type."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#InterfaceType)."] # [derive (Debug , Clone)] pub struct InterfaceType { # [doc = " The interfaces implemented by the interface."] pub implements : Vec < Positioned < Name > > , # [doc = " The fields of the interface type."] pub fields : Vec < Positioned < FieldDefinition > > , }
    };
}

InterfaceType!();