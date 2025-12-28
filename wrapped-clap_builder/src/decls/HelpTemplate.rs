macro_rules! deps {
    () => {
        Command!();
        Usage!();
        Styles!();
        StyledStr!();
    };
}

macro_rules! HelpTemplate {
    () => {
        deps!();
        # [doc = " Help template writer"] # [doc = ""] # [doc = " Wraps a writer stream providing different methods to generate help for `clap` objects."] pub (crate) struct HelpTemplate < 'cmd , 'writer > { writer : & 'writer mut StyledStr , cmd : & 'cmd Command , styles : & 'cmd Styles , usage : & 'cmd Usage < 'cmd > , next_line_help : bool , term_w : usize , use_long : bool , }
    };
}

HelpTemplate!()