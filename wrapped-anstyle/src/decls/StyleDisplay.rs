macro_rules! deps {
    () => {
        Style!();
    };
}

macro_rules! StyleDisplay {
    () => {
        deps!();
        # [derive (Copy , Clone , Default , Debug)] struct StyleDisplay (Style) ;
    };
}

StyleDisplay!()