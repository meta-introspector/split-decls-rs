macro_rules! deps {
    () => {
        Command!();
        FromArgMatches!();
    };
}

macro_rules! Subcommand {
    () => {
        deps!();
        # [doc = " Parse a sub-command into a user-defined enum."] # [doc = ""] # [doc = " Implementing this trait lets a parent container delegate subcommand behavior to `Self`."] # [doc = " with:"] # [doc = " - `#[command(subcommand)] field: SubCmd`: Attribute can be used with either struct fields or enum"] # [doc = "   variants that impl `Subcommand`."] # [doc = " - `#[command(flatten)] Variant(SubCmd)`: Attribute can only be used with enum variants that impl"] # [doc = "   `Subcommand`."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** Deriving requires the `derive` feature flag"] # [doc = ""] # [doc = " </div>"] pub trait Subcommand : FromArgMatches + Sized { # [doc = " Append to [`Command`] so it can instantiate `Self` via"] # [doc = " [`FromArgMatches::from_arg_matches_mut`]"] # [doc = ""] # [doc = " This is used to implement `#[command(flatten)]`"] # [doc = ""] # [doc = " See also [`CommandFactory::command`]."] fn augment_subcommands (cmd : Command) -> Command ; # [doc = " Append to [`Command`] so it can instantiate `self` via"] # [doc = " [`FromArgMatches::update_from_arg_matches_mut`]"] # [doc = ""] # [doc = " This is used to implement `#[command(flatten)]`"] # [doc = ""] # [doc = " See also [`CommandFactory::command_for_update`]."] fn augment_subcommands_for_update (cmd : Command) -> Command ; # [doc = " Test whether `Self` can parse a specific subcommand"] fn has_subcommand (name : & str) -> bool ; }
    };
}

Subcommand!();