macro_rules! deps {
    () => {
        NameRefKind!();
    };
}

macro_rules! NameRefContext {
    () => {
        deps!();
        # [doc = " The state of the NameRef we are completing."] # [derive (Debug)] pub (crate) struct NameRefContext < 'db > { # [doc = " NameRef syntax in the original file"] pub (crate) nameref : Option < ast :: NameRef > , pub (crate) kind : NameRefKind < 'db > , }
    };
}

NameRefContext!()