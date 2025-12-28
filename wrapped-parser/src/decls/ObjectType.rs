macro_rules! deps {
    () => {
        Positioned!();
        FieldDefinition!();
    };
}

macro_rules! ObjectType {
    () => {
        deps!();
        # [doc = " The definition of an object type."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#ObjectType)."] # [derive (Debug , Clone)] pub struct ObjectType { # [doc = " The interfaces implemented by the object."] pub implements : Vec < Positioned < Name > > , # [doc = " The fields of the object type."] pub fields : Vec < Positioned < FieldDefinition > > , }
    };
}

ObjectType!()