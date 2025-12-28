macro_rules! deps {
    () => {
        ModuleId!();
        VisibilityExplicitness!();
    };
}

macro_rules! Visibility {
    () => {
        deps!();
        # [doc = " Visibility of an item, with the path resolved."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum Visibility { # [doc = " Visibility is restricted to a certain module."] Module (ModuleId , VisibilityExplicitness) , # [doc = " Visibility is restricted to the crate."] PubCrate (Crate) , # [doc = " Visibility is unrestricted."] Public , }
    };
}

Visibility!();