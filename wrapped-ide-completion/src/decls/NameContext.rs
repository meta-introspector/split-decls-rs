macro_rules! deps {
    () => {
        NameKind!();
    };
}

macro_rules! NameContext {
    () => {
        deps!();
        # [doc = " The state of the name we are completing."] # [derive (Debug)] pub (crate) struct NameContext { # [allow (dead_code)] pub (crate) name : Option < ast :: Name > , pub (crate) kind : NameKind , }
    };
}

NameContext!()