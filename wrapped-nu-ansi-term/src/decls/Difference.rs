macro_rules! deps {
    () => {
        Style!();
    };
}

macro_rules! Difference {
    () => {
        deps!();
        # [doc = " When printing out one colored string followed by another, use one of"] # [doc = " these rules to figure out which *extra* control codes need to be sent."] # [derive (Eq , PartialEq , Clone , Copy , Debug)] pub enum Difference { # [doc = " Print out the control codes specified by this style to end up looking"] # [doc = " like the second string's styles."] ExtraStyles (Style) , # [doc = " Converting between these two is impossible, so just send a reset"] # [doc = " command and then the second string's styles."] Reset , # [doc = " The before style is exactly the same as the after style, so no further"] # [doc = " control codes need to be printed."] Empty , }
    };
}

Difference!();