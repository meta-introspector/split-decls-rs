macro_rules! deps {
    () => {
        SDLExportOptions!();
    };
}

macro_rules! write_description {
    () => {
        deps!();
        pub (super) fn write_description (sdl : & mut String , options : & SDLExportOptions , level : usize , description : & str ,) { let tabs = tab (options) . repeat (level) ; if options . prefer_single_line_descriptions && ! description . contains ('\n') { let description = description . replace ('"' , r#"\""#) ; writeln ! (sdl , "{tabs}\"{description}\"") . ok () ; } else { let description = description . replace ('\n' , & format ! ("\n{tabs}")) ; writeln ! (sdl , "{tabs}\"\"\"\n{tabs}{description}\n{tabs}\"\"\"") . ok () ; } }
    };
}

write_description!();