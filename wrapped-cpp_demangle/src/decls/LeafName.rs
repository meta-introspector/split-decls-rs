macro_rules! deps {
    () => {
        SourceName!();
        ClosureTypeName!();
        UnnamedTypeName!();
        CtorDtorName!();
    };
}

macro_rules! LeafName {
    () => {
        deps!();
        # [doc = " A leaf name is the part the name that describes some type or class without"] # [doc = " any leading namespace qualifiers."] # [doc = ""] # [doc = " This is used when figuring out how to format constructors and destructors,"] # [doc = " which are formatted as `gooble::dodo::Thing::~Thing()` but we don't have"] # [doc = " direct access to `Thing` in the `CtorDtorName` AST."] # [derive (Debug)] pub (crate) enum LeafName < 'a > { SourceName (& 'a SourceName) , WellKnownComponent (& 'a WellKnownComponent) , Closure (& 'a ClosureTypeName) , UnnamedType (& 'a UnnamedTypeName) , }
    };
}

LeafName!();