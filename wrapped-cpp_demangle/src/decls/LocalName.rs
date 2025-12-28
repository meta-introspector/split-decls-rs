macro_rules! deps {
    () => {
        Encoding!();
        Name!();
        Discriminator!();
    };
}

macro_rules! LocalName {
    () => {
        deps!();
        # [doc = " The `<local-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <local-name> := Z <function encoding> E <entity name> [<discriminator>]"] # [doc = "              := Z <function encoding> E s [<discriminator>]"] # [doc = "              := Z <function encoding> Ed [ <parameter number> ] _ <entity name>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum LocalName { # [doc = " The mangling of the enclosing function, the mangling of the entity"] # [doc = " relative to the function, and an optional discriminator."] Relative (Box < Encoding > , Option < Box < Name > > , Option < Discriminator >) , # [doc = " A default argument in a class definition."] Default (Box < Encoding > , Option < usize > , Box < Name >) , }
    };
}

LocalName!()