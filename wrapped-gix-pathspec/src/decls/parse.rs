macro_rules! deps {
    () => {
        Error!();
        Pattern!();
        Defaults!();
    };
}

macro_rules! parse {
    () => {
        deps!();
        # [doc = " Parse a git-style pathspec into a [`Pattern`],"] # [doc = " setting the given `default` values in case these aren't specified in `input`."] # [doc = ""] # [doc = " Note that empty [paths](Pattern::path) are allowed here, and generally some processing has to be performed."] pub fn parse (input : & [u8] , default : Defaults) -> Result < Pattern , parse :: Error > { Pattern :: from_bytes (input , default) }
    };
}

parse!();