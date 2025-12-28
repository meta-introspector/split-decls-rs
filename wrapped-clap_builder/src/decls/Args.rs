macro_rules! deps {
    () => {
        Id!();
        FromArgMatches!();
        Command!();
    };
}

macro_rules! Args {
    () => {
        deps!();
        # [doc = " Parse a set of arguments into a user-defined container."] # [doc = ""] # [doc = " Implementing this trait lets a parent container delegate argument parsing behavior to `Self`."] # [doc = " with:"] # [doc = " - `#[command(flatten)] args: ChildArgs`: Attribute can only be used with struct fields that impl"] # [doc = "   `Args`."] # [doc = " - `Variant(ChildArgs)`: No attribute is used with enum variants that impl `Args`."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** Deriving requires the `derive` feature flag"] # [doc = ""] # [doc = " </div>"] pub trait Args : FromArgMatches + Sized { # [doc = " Report the [`ArgGroup::id`][crate::ArgGroup::id] for this set of arguments"] fn group_id () -> Option < crate :: Id > { None } # [doc = " Append to [`Command`] so it can instantiate `Self` via"] # [doc = " [`FromArgMatches::from_arg_matches_mut`]"] # [doc = ""] # [doc = " This is used to implement `#[command(flatten)]`"] # [doc = ""] # [doc = " See also [`CommandFactory::command`]."] fn augment_args (cmd : Command) -> Command ; # [doc = " Append to [`Command`] so it can instantiate `self` via"] # [doc = " [`FromArgMatches::update_from_arg_matches_mut`]"] # [doc = ""] # [doc = " This is used to implement `#[command(flatten)]`"] # [doc = ""] # [doc = " See also [`CommandFactory::command_for_update`]."] fn augment_args_for_update (cmd : Command) -> Command ; }
    };
}

Args!()