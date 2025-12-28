macro_rules! deps {
    () => {
        ValueRange!();
        Extensions!();
        Id!();
        ValueParser!();
        ArgAction!();
        StyledStr!();
        ArgPredicate!();
        OsStr!();
        ArgFlags!();
        Str!();
    };
}

macro_rules! Arg {
    () => {
        deps!();
        # [doc = " The abstract representation of a command line argument. Used to set all the options and"] # [doc = " relationships that define a valid argument for the program."] # [doc = ""] # [doc = " There are two methods for constructing [`Arg`]s, using the builder pattern and setting options"] # [doc = " manually, or using a usage string which is far less verbose but has fewer options. You can also"] # [doc = " use a combination of the two methods to achieve the best of both worlds."] # [doc = ""] # [doc = " - [Basic API][crate::Arg#basic-api]"] # [doc = " - [Value Handling][crate::Arg#value-handling]"] # [doc = " - [Help][crate::Arg#help-1]"] # [doc = " - [Advanced Argument Relations][crate::Arg#advanced-argument-relations]"] # [doc = " - [Reflection][crate::Arg#reflection]"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::{Arg, arg, ArgAction};"] # [doc = " // Using the traditional builder pattern and setting each option manually"] # [doc = " let cfg = Arg::new(\"config\")"] # [doc = "       .short('c')"] # [doc = "       .long(\"config\")"] # [doc = "       .action(ArgAction::Set)"] # [doc = "       .value_name(\"FILE\")"] # [doc = "       .help(\"Provides a config file to myprog\");"] # [doc = " // Using a usage string (setting a similar argument to the one above)"] # [doc = " let input = arg!(-i --input <FILE> \"Provides an input file to the program\");"] # [doc = " ```"] # [derive (Default , Clone)] pub struct Arg { pub (crate) id : Id , pub (crate) help : Option < StyledStr > , pub (crate) long_help : Option < StyledStr > , pub (crate) action : Option < ArgAction > , pub (crate) value_parser : Option < super :: ValueParser > , pub (crate) blacklist : Vec < Id > , pub (crate) settings : ArgFlags , pub (crate) overrides : Vec < Id > , pub (crate) groups : Vec < Id > , pub (crate) requires : Vec < (ArgPredicate , Id) > , pub (crate) r_ifs : Vec < (Id , OsStr) > , pub (crate) r_ifs_all : Vec < (Id , OsStr) > , pub (crate) r_unless : Vec < Id > , pub (crate) r_unless_all : Vec < Id > , pub (crate) short : Option < char > , pub (crate) long : Option < Str > , pub (crate) aliases : Vec < (Str , bool) > , pub (crate) short_aliases : Vec < (char , bool) > , pub (crate) disp_ord : Option < usize > , pub (crate) val_names : Vec < Str > , pub (crate) num_vals : Option < ValueRange > , pub (crate) val_delim : Option < char > , pub (crate) default_vals : Vec < OsStr > , pub (crate) default_vals_ifs : Vec < (Id , ArgPredicate , Option < Vec < OsStr > >) > , pub (crate) default_missing_vals : Vec < OsStr > , # [cfg (feature = "env")] pub (crate) env : Option < (OsStr , Option < OsString >) > , pub (crate) terminator : Option < Str > , pub (crate) index : Option < usize > , pub (crate) help_heading : Option < Option < Str > > , pub (crate) ext : Extensions , }
    };
}

Arg!();