macro_rules! deps {
    () => {
        Args!();
        ArgMatches!();
        Result!();
        Subcommand!();
        Error!();
        Parser!();
    };
}

macro_rules! FromArgMatches {
    () => {
        deps!();
        # [doc = " Converts an instance of [`ArgMatches`] to a user-defined container."] # [doc = ""] # [doc = " Derived as part of [`Parser`], [`Args`], and [`Subcommand`]."] pub trait FromArgMatches : Sized { # [doc = " Instantiate `Self` from [`ArgMatches`], parsing the arguments as needed."] # [doc = ""] # [doc = " Motivation: If our application had two CLI options, `--name"] # [doc = " <STRING>` and the flag `--debug`, we may create a struct as follows:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(feature = \"derive\")] {"] # [doc = " struct Context {"] # [doc = "     name: String,"] # [doc = "     debug: bool"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " We then need to convert the `ArgMatches` that `clap` generated into our struct."] # [doc = " `from_arg_matches` serves as the equivalent of:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(feature = \"derive\")] {"] # [doc = " # use clap::ArgMatches;"] # [doc = " # struct Context {"] # [doc = " #   name: String,"] # [doc = " #   debug: bool"] # [doc = " # }"] # [doc = " impl From<ArgMatches> for Context {"] # [doc = "    fn from(m: ArgMatches) -> Self {"] # [doc = "        Context {"] # [doc = "            name: m.get_one::<String>(\"name\").unwrap().clone(),"] # [doc = "            debug: m.get_flag(\"debug\"),"] # [doc = "        }"] # [doc = "    }"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] fn from_arg_matches (matches : & ArgMatches) -> Result < Self , Error > ; # [doc = " Instantiate `Self` from [`ArgMatches`], parsing the arguments as needed."] # [doc = ""] # [doc = " Motivation: If our application had two CLI options, `--name"] # [doc = " <STRING>` and the flag `--debug`, we may create a struct as follows:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(feature = \"derive\")] {"] # [doc = " struct Context {"] # [doc = "     name: String,"] # [doc = "     debug: bool"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " We then need to convert the `ArgMatches` that `clap` generated into our struct."] # [doc = " `from_arg_matches_mut` serves as the equivalent of:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(feature = \"derive\")] {"] # [doc = " # use clap::ArgMatches;"] # [doc = " # struct Context {"] # [doc = " #   name: String,"] # [doc = " #   debug: bool"] # [doc = " # }"] # [doc = " impl From<ArgMatches> for Context {"] # [doc = "    fn from(m: ArgMatches) -> Self {"] # [doc = "        Context {"] # [doc = "            name: m.get_one::<String>(\"name\").unwrap().to_string(),"] # [doc = "            debug: m.get_flag(\"debug\"),"] # [doc = "        }"] # [doc = "    }"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] fn from_arg_matches_mut (matches : & mut ArgMatches) -> Result < Self , Error > { Self :: from_arg_matches (matches) } # [doc = " Assign values from `ArgMatches` to `self`."] fn update_from_arg_matches (& mut self , matches : & ArgMatches) -> Result < () , Error > ; # [doc = " Assign values from `ArgMatches` to `self`."] fn update_from_arg_matches_mut (& mut self , matches : & mut ArgMatches) -> Result < () , Error > { self . update_from_arg_matches (matches) } }
    };
}

FromArgMatches!();