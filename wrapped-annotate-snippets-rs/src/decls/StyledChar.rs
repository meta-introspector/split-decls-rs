macro_rules! deps {
    () => {
        ElementStyle!();
    };
}

macro_rules! StyledChar {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialEq)] pub (crate) struct StyledChar { ch : char , style : ElementStyle , }
    };
}

StyledChar!();