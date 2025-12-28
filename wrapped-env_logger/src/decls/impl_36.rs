macro_rules! deps {
    () => {
        WriteStyle!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        # [cfg (feature = "color")] impl From < anstream :: ColorChoice > for WriteStyle { fn from (choice : anstream :: ColorChoice) -> Self { match choice { anstream :: ColorChoice :: Auto => Self :: Auto , anstream :: ColorChoice :: Always => Self :: Always , anstream :: ColorChoice :: AlwaysAnsi => Self :: Always , anstream :: ColorChoice :: Never => Self :: Never , } } }
    };
}

impl_36!()