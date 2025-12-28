macro_rules! deps {
    () => {
        Error!();
        Options!();
        Outcome!();
    };
}

macro_rules! objects {
    () => {
        deps!();
        # [doc = ""] pub mod objects { pub use super :: objects_impl :: { Error , ObjectExpansion , Options , Outcome } ; }
    };
}

objects!();