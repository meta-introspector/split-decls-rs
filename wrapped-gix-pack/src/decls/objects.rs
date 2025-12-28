macro_rules! deps {
    () => {
        Options!();
        Outcome!();
        Error!();
    };
}

macro_rules! objects {
    () => {
        deps!();
        # [doc = ""] pub mod objects { pub use super :: objects_impl :: { Error , ObjectExpansion , Options , Outcome } ; }
    };
}

objects!()