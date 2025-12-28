macro_rules! IdentifierKind {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq)] pub enum IdentifierKind { Ident , Lifetime , Underscore , LowercaseSelf , }
    };
}

IdentifierKind!()