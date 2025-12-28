macro_rules! deps {
    () => {
        Encoding!();
        Attribute!();
        Result!();
        AttributeSpecification!();
        AttributeValue!();
    };
}

macro_rules! impl_807 {
    () => {
        deps!();
        impl Attribute { # [doc = " Get the name of this attribute."] # [inline] pub fn name (& self) -> constants :: DwAt { self . name } # [doc = " Get the value of this attribute."] # [inline] pub fn get (& self) -> & AttributeValue { & self . value } # [doc = " Set the value of this attribute."] # [inline] pub fn set (& mut self , value : AttributeValue) { self . value = value ; } # [doc = " Return the type specification for this attribute."] fn specification (& self , encoding : Encoding) -> Result < AttributeSpecification > { Ok (AttributeSpecification :: new (self . name , self . value . form (encoding) ? ,)) } }
    };
}

impl_807!();