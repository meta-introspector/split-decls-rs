macro_rules! Assists {
    () => {
        pub (crate) struct Assists { file : FileId , resolve : AssistResolveStrategy , buf : Vec < Assist > , allowed : Option < Vec < AssistKind > > , }
    };
}

Assists!();