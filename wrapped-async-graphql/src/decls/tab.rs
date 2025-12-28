macro_rules! deps {
    () => {
        SDLExportOptions!();
    };
}

macro_rules! tab {
    () => {
        deps!();
        fn tab (options : & SDLExportOptions) -> String { if options . use_space_ident { " " . repeat (options . indent_width . into ()) } else { "\t" . to_string () } }
    };
}

tab!();