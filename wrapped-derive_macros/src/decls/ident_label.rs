macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! ident_label {
    () => {
        deps!();
        fn ident_label (ident : & Ident) -> Label { Label :: Implicit ({ let ident = ident . unraw () . to_string () ; quote ! (# ident) }) }
    };
}

ident_label!();