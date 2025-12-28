macro_rules! UnreachableLabel {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct UnreachableLabel { pub node : InFile < AstPtr < ast :: Lifetime > > , pub name : Name , }
    };
}

UnreachableLabel!()