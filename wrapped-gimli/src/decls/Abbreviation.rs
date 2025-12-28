macro_rules! deps {
    () => {
        AttributeSpecification!();
    };
}

macro_rules! Abbreviation {
    () => {
        deps!();
        # [doc = " An abbreviation describes the shape of a `DebuggingInformationEntry`'s type:"] # [doc = " its tag type, whether it has children, and its set of attributes."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub (crate) struct Abbreviation { tag : constants :: DwTag , has_children : bool , attributes : Vec < AttributeSpecification > , }
    };
}

Abbreviation!();