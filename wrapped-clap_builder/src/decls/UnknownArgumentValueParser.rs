macro_rules! deps {
    () => {
        Str!();
        StyledStr!();
    };
}

macro_rules! UnknownArgumentValueParser {
    () => {
        deps!();
        # [doc = " When encountered, report [`ErrorKind::UnknownArgument`][crate::error::ErrorKind::UnknownArgument]"] # [doc = ""] # [doc = " Useful to help users migrate, either from old versions or similar tools."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::Command;"] # [doc = " # use clap::Arg;"] # [doc = " let cmd = Command::new(\"mycmd\")"] # [doc = "     .args(["] # [doc = "         Arg::new(\"current-dir\")"] # [doc = "             .short('C'),"] # [doc = "         Arg::new(\"current-dir-unknown\")"] # [doc = "             .long(\"cwd\")"] # [doc = "             .aliases([\"current-dir\", \"directory\", \"working-directory\", \"root\"])"] # [doc = "             .value_parser(clap::builder::UnknownArgumentValueParser::suggest_arg(\"-C\"))"] # [doc = "             .hide(true),"] # [doc = "     ]);"] # [doc = ""] # [doc = " // Use a supported version of the argument"] # [doc = " let matches = cmd.clone().try_get_matches_from([\"mycmd\", \"-C\", \"..\"]).unwrap();"] # [doc = " assert!(matches.contains_id(\"current-dir\"));"] # [doc = " assert_eq!("] # [doc = "     matches.get_many::<String>(\"current-dir\").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),"] # [doc = "     vec![\"..\"]"] # [doc = " );"] # [doc = ""] # [doc = " // Use one of the invalid versions"] # [doc = " let err = cmd.try_get_matches_from([\"mycmd\", \"--cwd\", \"..\"]).unwrap_err();"] # [doc = " assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct UnknownArgumentValueParser { arg : Option < Str > , suggestions : Vec < StyledStr > , }
    };
}

UnknownArgumentValueParser!()