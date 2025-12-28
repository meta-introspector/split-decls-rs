macro_rules! deps {
    () => {
        Assignments!();
        AttributeId!();
        MetadataCollection!();
    };
}

macro_rules! Metadata {
    () => {
        deps!();
        # [doc = " Metadata associated with an attribute or macro name."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub struct Metadata { # [doc = " The id to uniquely identify an attribute in the [MetadataCollection]."] pub id : AttributeId , # [doc = " If non-zero in length, this entry belongs to a macro which resolves to these attribute names."] pub macro_attributes : Assignments , }
    };
}

Metadata!()