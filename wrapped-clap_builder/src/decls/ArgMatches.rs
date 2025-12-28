macro_rules! deps {
    () => {
        Str!();
        ArgAction!();
        MatchedArg!();
        Id!();
        Arg!();
        FlatMap!();
        SubCommand!();
    };
}

macro_rules! ArgMatches {
    () => {
        deps!();
        # [doc = " Container for parse results."] # [doc = ""] # [doc = " Used to get information about the arguments that were supplied to the program at runtime by"] # [doc = " the user. New instances of this struct are obtained by using the [`Command::get_matches`] family of"] # [doc = " methods."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::{Command, Arg, ArgAction};"] # [doc = " # use clap::parser::ValueSource;"] # [doc = " let matches = Command::new(\"MyApp\")"] # [doc = "     .arg(Arg::new(\"out\")"] # [doc = "         .long(\"output\")"] # [doc = "         .required(true)"] # [doc = "         .action(ArgAction::Set)"] # [doc = "         .default_value(\"-\"))"] # [doc = "     .arg(Arg::new(\"cfg\")"] # [doc = "         .short('c')"] # [doc = "         .action(ArgAction::Set))"] # [doc = "     .get_matches(); // builds the instance of ArgMatches"] # [doc = ""] # [doc = " // to get information about the \"cfg\" argument we created, such as the value supplied we use"] # [doc = " // various ArgMatches methods, such as [ArgMatches::get_one]"] # [doc = " if let Some(c) = matches.get_one::<String>(\"cfg\") {"] # [doc = "     println!(\"Value for -c: {c}\");"] # [doc = " }"] # [doc = ""] # [doc = " // The ArgMatches::get_one method returns an Option because the user may not have supplied"] # [doc = " // that argument at runtime. But if we specified that the argument was \"required\" as we did"] # [doc = " // with the \"out\" argument, we can safely unwrap because `clap` verifies that was actually"] # [doc = " // used at runtime."] # [doc = " println!(\"Value for --output: {}\", matches.get_one::<String>(\"out\").unwrap());"] # [doc = ""] # [doc = " // You can check the presence of an argument's values"] # [doc = " if matches.contains_id(\"out\") {"] # [doc = "     // However, if you want to know where the value came from"] # [doc = "     if matches.value_source(\"out\").expect(\"checked contains_id\") == ValueSource::CommandLine {"] # [doc = "         println!(\"`out` set by user\");"] # [doc = "     } else {"] # [doc = "         println!(\"`out` is defaulted\");"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " [`Command::get_matches`]: crate::Command::get_matches()"] # [derive (Debug , Clone , Default , PartialEq , Eq)] pub struct ArgMatches { # [cfg (debug_assertions)] pub (crate) valid_args : Vec < Id > , # [cfg (debug_assertions)] pub (crate) valid_subcommands : Vec < Str > , pub (crate) args : FlatMap < Id , MatchedArg > , pub (crate) subcommand : Option < Box < SubCommand > > , }
    };
}

ArgMatches!()