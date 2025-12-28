macro_rules! deps {
    () => {
        UnscopedName!();
    };
}

macro_rules! UnscopedTemplateName {
    () => {
        deps!();
        # [doc = " The `<unscoped-template-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <unscoped-template-name> ::= <unscoped-name>"] # [doc = "                          ::= <substitution>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct UnscopedTemplateName (UnscopedName) ;
    };
}

UnscopedTemplateName!();