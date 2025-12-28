macro_rules! macro_2 {
    () => {
        feature ! { #! [feature = "env"] pub mod env ; }
    };
}

macro_2!()