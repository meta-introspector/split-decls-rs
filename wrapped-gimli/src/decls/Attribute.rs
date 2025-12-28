macro_rules! deps {
    () => {
        AttributeValue!();
        DebuggingInformationEntry!();
    };
}

macro_rules! Attribute {
    () => {
        deps!();
        # [doc = " An attribute in a `DebuggingInformationEntry`, consisting of a name and"] # [doc = " associated value."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct Attribute { name : constants :: DwAt , value : AttributeValue , }
    };
}

Attribute!();