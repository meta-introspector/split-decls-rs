macro_rules! deps {
    () => {
        Id!();
        Level!();
    };
}

macro_rules! MessageOrTitle {
    () => {
        deps!();
        trait MessageOrTitle { fn level (& self) -> & Level < '_ > ; fn id (& self) -> Option < & Id < '_ > > ; fn text (& self) -> & str ; fn allows_styling (& self) -> bool ; }
    };
}

MessageOrTitle!();