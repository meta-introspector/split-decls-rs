macro_rules! deps {
    () => {
        Mode!();
    };
}

macro_rules! Options {
    () => {
        deps!();
        # [doc = " The options used in `[ask()]`."] # [derive (Default , Clone)] pub struct Options < 'a > { # [doc = " The path or name (for lookup in `PATH`) to the askpass program to call before prompting the user."] # [doc = ""] # [doc = " It's called like this `askpass <prompt>`, but note that it won't know if the input should be hidden or not."] pub askpass : Option < Cow < 'a , Path > > , # [doc = " The way the user is prompted."] pub mode : Mode , }
    };
}

Options!()