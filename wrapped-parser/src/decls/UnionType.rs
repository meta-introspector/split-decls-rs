macro_rules! deps {
    () => {
        Positioned!();
    };
}

macro_rules! UnionType {
    () => {
        deps!();
        # [doc = " The definition of a union type."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#UnionType)."] # [derive (Debug , Clone)] pub struct UnionType { # [doc = " The member types of the union."] pub members : Vec < Positioned < Name > > , }
    };
}

UnionType!();