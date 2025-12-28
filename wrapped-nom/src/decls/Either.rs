macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Either {
    () => {
        deps!();
        # [doc = " Alternate between two Parser implementations with the same result type."] pub (crate) enum Either < F , G > { Left (F) , Right (G) , }
    };
}

Either!()