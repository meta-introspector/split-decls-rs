macro_rules! deps {
    () => {
        Value!();
        Type!();
    };
}

macro_rules! Attribute {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub struct Attribute { pub Parent : HasAttribute , pub Type : AttributeType , pub Value : id :: BlobId , }
    };
}

Attribute!();