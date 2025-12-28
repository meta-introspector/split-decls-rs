macro_rules! Identifier {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) enum Identifier { Short , Long , Index , }
    };
}

Identifier!()