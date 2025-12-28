macro_rules! deps {
    () => {
        Reader!();
        Unit!();
        Dwarf!();
    };
}

macro_rules! UnitRef {
    () => {
        deps!();
        # [doc = " A reference to a `Unit` and its associated `Dwarf`."] # [doc = ""] # [doc = " These often need to be passed around together, so this struct makes that easier."] # [doc = ""] # [doc = " It implements `Deref` to `&'a Unit`, so you can use it as if it were a `Unit`."] # [doc = " It also implements methods that correspond to methods on `Dwarf` that take a `Unit`."] # [derive (Debug)] pub struct UnitRef < 'a , R : Reader > { # [doc = " The `Dwarf` that contains the unit."] pub dwarf : & 'a Dwarf < R > , # [doc = " The `Unit` being referenced."] pub unit : & 'a Unit < R > , }
    };
}

UnitRef!();