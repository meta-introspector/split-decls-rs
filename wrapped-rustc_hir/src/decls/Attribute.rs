macro_rules! deps {
    () => {
        AttributeKind!();
        AttrItem!();
    };
}

macro_rules! Attribute {
    () => {
        deps!();
        # [derive (Clone , Debug , Encodable , Decodable , HashStable_Generic)] pub enum Attribute { # [doc = " A parsed built-in attribute."] # [doc = ""] # [doc = " Each attribute has a span connected to it. However, you must be somewhat careful using it."] # [doc = " That's because sometimes we merge multiple attributes together, like when an item has"] # [doc = " multiple `repr` attributes. In this case the span might not be very useful."] Parsed (AttributeKind) , # [doc = " An attribute that could not be parsed, out of a token-like representation."] # [doc = " This is the case for custom tool attributes."] Unparsed (Box < AttrItem >) , }
    };
}

Attribute!()