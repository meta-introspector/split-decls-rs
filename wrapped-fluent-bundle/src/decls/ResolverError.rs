macro_rules! deps {
    () => {
        WriteValue!();
        ReferenceKind!();
    };
}

macro_rules! ResolverError {
    () => {
        deps!();
        # [doc = " Errors generated during the process of resolving a fluent message into a string."] # [doc = " This process takes place in the `write` method of the `WriteValue` trait."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum ResolverError { Reference (ReferenceKind) , NoValue (String) , MissingDefault , Cyclic , TooManyPlaceables , }
    };
}

ResolverError!()