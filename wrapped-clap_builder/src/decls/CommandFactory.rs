macro_rules! deps {
    () => {
        Parser!();
        Command!();
    };
}

macro_rules! CommandFactory {
    () => {
        deps!();
        # [doc = " Create a [`Command`] relevant for a user-defined container."] # [doc = ""] # [doc = " Derived as part of [`Parser`]."] pub trait CommandFactory : Sized { # [doc = " Build a [`Command`] that can instantiate `Self`."] # [doc = ""] # [doc = " See [`FromArgMatches::from_arg_matches_mut`] for instantiating `Self`."] fn command () -> Command ; # [doc = " Build a [`Command`] that can update `self`."] # [doc = ""] # [doc = " See [`FromArgMatches::update_from_arg_matches_mut`] for updating `self`."] fn command_for_update () -> Command ; }
    };
}

CommandFactory!();