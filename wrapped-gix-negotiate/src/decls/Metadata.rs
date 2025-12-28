macro_rules! deps {
    () => {
        Negotiator!();
    };
}

macro_rules! Metadata {
    () => {
        deps!();
        # [doc = " Additional data to store with each commit when used by any of our algorithms."] # [doc = ""] # [doc = " It's shared among those who use the [`Negotiator`] trait, and all implementations of it."] # [derive (Default , Debug , Copy , Clone)] pub struct Metadata { # [doc = " Used by `skipping`."] # [doc = " Only used if commit is not COMMON"] pub original_ttl : u16 , # [doc = " Used by `skipping`."] pub ttl : u16 , # [doc = " Additional information about each commit"] pub flags : Flags , }
    };
}

Metadata!();