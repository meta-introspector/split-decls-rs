macro_rules! deps {
    () => {
        CustomFormat!();
    };
}

macro_rules! GITOXIDE {
    () => {
        deps!();
        # [doc = " E.g. `Thu Sep 04 2022 10:45:06 -0400`, like the git `DEFAULT`, but with the year and time fields swapped."] pub const GITOXIDE : CustomFormat = CustomFormat ("%a %b %d %Y %H:%M:%S %z") ;
    };
}

GITOXIDE!();