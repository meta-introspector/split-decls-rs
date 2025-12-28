macro_rules! Id {
    () => {
        # [doc = " `Id` is a Graphviz `ID`."] pub struct Id < 'a > { name : Cow < 'a , str > , }
    };
}

Id!()