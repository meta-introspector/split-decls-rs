macro_rules! IdentListAttribute {
    () => {
        struct IdentListAttribute { idents : Punctuated < Ident , Token ! [,] > , }
    };
}

IdentListAttribute!()