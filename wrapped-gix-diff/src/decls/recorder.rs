macro_rules! deps {
    () => {
        Visit!();
    };
}

macro_rules! recorder {
    () => {
        deps!();
        # [doc = " Useful for use as delegate implementing [`Visit`] to keep track of all seen changes. Useful for debugging or printing primarily."] pub mod recorder ;
    };
}

recorder!();