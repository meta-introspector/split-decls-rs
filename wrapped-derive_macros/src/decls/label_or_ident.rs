macro_rules! deps {
    () => {
        Label!();
        LabelValue!();
    };
}

macro_rules! label_or_ident {
    () => {
        deps!();
        pub (crate) fn label_or_ident < 'a > (explicit : Option < LabelValue > , ident : & Ident) -> Label { explicit . map (explicit_label) . unwrap_or_else (| | ident_label (ident)) }
    };
}

label_or_ident!();