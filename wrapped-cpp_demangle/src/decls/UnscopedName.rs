macro_rules! deps {
    () => {
        UnqualifiedName!();
    };
}

macro_rules! UnscopedName {
    () => {
        deps!();
        # [doc = " The `<unscoped-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <unscoped-name> ::= <unqualified-name>"] # [doc = "                 ::= St <unqualified-name>   # ::std::"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum UnscopedName { # [doc = " An unqualified name."] Unqualified (UnqualifiedName) , # [doc = " A name within the `std::` namespace."] Std (UnqualifiedName) , }
    };
}

UnscopedName!()