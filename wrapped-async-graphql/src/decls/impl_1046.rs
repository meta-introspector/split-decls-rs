macro_rules! deps {
    () => {
        SDLExportOptions!();
        MetaDirective!();
        MetaInputValue!();
    };
}

macro_rules! impl_1046 {
    () => {
        deps!();
        impl MetaDirective { pub (crate) fn sdl (& self , options : & SDLExportOptions) -> String { let mut sdl = String :: new () ; if let Some (description) = & self . description { self :: export_sdl :: write_description (& mut sdl , options , 0 , description) ; } write ! (sdl , "directive @{}" , self . name) . ok () ; if ! self . args . is_empty () { let args = self . args . values () . map (| value | self . argument_sdl (value)) . collect :: < Vec < _ > > () . join (", ") ; write ! (sdl , "({})" , args) . ok () ; } let locations = self . locations . iter () . map (| location | location . to_value () . to_string ()) . collect :: < Vec < _ > > () . join (" | ") ; write ! (sdl , " on {}" , locations) . ok () ; sdl } pub (crate) fn argument_sdl (& self , argument : & MetaInputValue) -> String { let argument_default = match & argument . default_value { Some (default) => format ! (" = {default}") , None => "" . to_string () , } ; format ! ("{}: {}{}" , argument . name , argument . ty , argument_default) } }
    };
}

impl_1046!()