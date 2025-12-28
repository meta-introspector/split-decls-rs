macro_rules! deps {
    () => {
        ObjectMapEntry!();
        ObjectMapFile!();
        SymbolMap!();
    };
}

macro_rules! ObjectMap {
    () => {
        deps!();
        # [doc = " A map from addresses to symbol names and object files."] # [doc = ""] # [doc = " This is derived from STAB entries in Mach-O files."] # [doc = ""] # [doc = " Returned by [`Object::object_map`]."] # [derive (Debug , Default , Clone)] pub struct ObjectMap < 'data > { symbols : SymbolMap < ObjectMapEntry < 'data > > , objects : Vec < ObjectMapFile < 'data > > , }
    };
}

ObjectMap!()