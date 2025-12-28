macro_rules! deps {
    () => {
        MetaDirectiveInvocation!();
    };
}

macro_rules! impl_1031 {
    () => {
        deps!();
        impl MetaDirectiveInvocation { pub fn sdl (& self) -> String { let formatted_args = if self . args . is_empty () { String :: new () } else { format ! ("({})" , self . args . iter () . map (| (name , value) | format ! ("{}: {}" , name , value)) . collect ::< Vec < _ >> () . join (", ")) } ; format ! ("@{}{}" , self . name , formatted_args) } }
    };
}

impl_1031!()