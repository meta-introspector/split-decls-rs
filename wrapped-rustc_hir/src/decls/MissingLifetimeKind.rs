macro_rules! MissingLifetimeKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , HashStable_Generic , Debug)] pub enum MissingLifetimeKind { # [doc = " An explicit `'_`."] Underscore , # [doc = " An elided lifetime `&' ty`."] Ampersand , # [doc = " An elided lifetime in brackets with written brackets."] Comma , # [doc = " An elided lifetime with elided brackets."] Brackets , }
    };
}

MissingLifetimeKind!();