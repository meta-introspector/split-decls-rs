macro_rules! deps {
    () => {
        TypeDirectiveLocation!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl TypeDirectiveLocation { pub fn location_trait_identifier (& self) -> Ident { format_ident ! ("Directive_At_{}" , self . to_string ()) } }
    };
}

impl_60!()