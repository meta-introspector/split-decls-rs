macro_rules! deps {
    () => {
        MockableStruct!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl MockableStruct { # [doc = " Does this struct derive Debug?"] pub fn derives_debug (& self) -> bool { self . attrs . iter () . any (| attr | { let mut derive_debug = false ; if attr . path () . is_ident ("derive") { attr . parse_nested_meta (| meta | { if meta . path . is_ident ("Debug") { derive_debug = true ; } Ok (()) }) . unwrap () ; } derive_debug }) } }
    };
}

impl_103!()