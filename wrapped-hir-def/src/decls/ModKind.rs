macro_rules! ModKind {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub (crate) enum ModKind { # [doc = " `mod m { ... }`"] Inline { items : Box < [ModItemId] > } , # [doc = " `mod m;`"] Outline , }
    };
}

ModKind!();