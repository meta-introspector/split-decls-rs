macro_rules! deps {
    () => {
        LabelValue!();
        Label!();
    };
}

macro_rules! optional_label_or_ident {
    () => {
        deps!();
        pub (crate) fn optional_label_or_ident < 'a > (explicit : Option < LabelValue > , ident : Option < & Ident > ,) -> Option < Label > { explicit . map (explicit_label) . or_else (| | ident . map (ident_label)) }
    };
}

optional_label_or_ident!();