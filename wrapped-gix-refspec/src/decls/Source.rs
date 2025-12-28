macro_rules! deps {
    () => {
        SourceRef!();
    };
}

macro_rules! Source {
    () => {
        deps!();
        # [doc = " The source (or left-hand) side of a mapping, which owns its name."] pub type Source = SourceRef < 'static > ;
    };
}

Source!();