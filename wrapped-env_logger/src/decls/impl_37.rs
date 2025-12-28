macro_rules! deps {
    () => {
        WriteStyle!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        # [cfg (feature = "color")] impl From < WriteStyle > for anstream :: ColorChoice { fn from (choice : WriteStyle) -> Self { match choice { WriteStyle :: Auto => anstream :: ColorChoice :: Auto , WriteStyle :: Always => anstream :: ColorChoice :: Always , WriteStyle :: Never => anstream :: ColorChoice :: Never , } } }
    };
}

impl_37!();