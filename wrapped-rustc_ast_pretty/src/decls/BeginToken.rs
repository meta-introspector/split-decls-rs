macro_rules! deps {
    () => {
        IndentStyle!();
        Breaks!();
    };
}

macro_rules! BeginToken {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq)] pub (crate) struct BeginToken { indent : IndentStyle , breaks : Breaks , }
    };
}

BeginToken!()