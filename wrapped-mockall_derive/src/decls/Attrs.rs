macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! Attrs {
    () => {
        deps!();
        # [doc = " automock attributes"] # [derive (Debug , Default)] pub (crate) struct Attrs { pub target : Option < Ident > , pub attrs : HashMap < Ident , Type > , }
    };
}

Attrs!();