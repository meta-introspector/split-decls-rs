macro_rules! UndeclaredLabel {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct UndeclaredLabel { pub node : InFile < AstPtr < ast :: Lifetime > > , pub name : Name , }
    };
}

UndeclaredLabel!()