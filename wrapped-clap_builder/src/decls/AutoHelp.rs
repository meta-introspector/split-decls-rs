macro_rules! deps {
    () => {
        HelpTemplate!();
    };
}

macro_rules! AutoHelp {
    () => {
        deps!();
        # [doc = " `clap` auto-generated help writer"] pub (crate) struct AutoHelp < 'cmd , 'writer > { template : HelpTemplate < 'cmd , 'writer > , }
    };
}

AutoHelp!()