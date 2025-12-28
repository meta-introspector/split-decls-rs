macro_rules! macro_4 {
    () => {
        feature ! { #! [feature = "feature"] # [deny (missing_docs)] pub mod features ; }
    };
}

macro_4!()