macro_rules! deps {
    () => {
        NavigationTarget!();
    };
}

macro_rules! Declaration {
    () => {
        deps!();
        # [doc = " Information about the declaration site of a searched item."] # [derive (Debug , Clone , UpmapFromRaFixture)] pub struct Declaration { # [doc = " Navigation information to jump to the declaration"] pub nav : NavigationTarget , # [doc = " Whether the declared item is mutable (relevant for variables)"] pub is_mut : bool , }
    };
}

Declaration!()