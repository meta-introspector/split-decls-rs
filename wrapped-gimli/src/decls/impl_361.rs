macro_rules! deps {
    () => {
        ArangeHeaderIter!();
        ArangeHeader!();
        DebugAranges!();
        Reader!();
        DebugArangesOffset!();
        Result!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        impl < R : Reader > DebugAranges < R > { # [doc = " Iterate the sets of entries in the `.debug_aranges` section."] # [doc = ""] # [doc = " Each set of entries belongs to a single unit."] pub fn headers (& self) -> ArangeHeaderIter < R > { ArangeHeaderIter { input : self . section . clone () , offset : DebugArangesOffset (R :: Offset :: from_u8 (0)) , } } # [doc = " Get the header at the given offset."] pub fn header (& self , offset : DebugArangesOffset < R :: Offset >) -> Result < ArangeHeader < R > > { let mut input = self . section . clone () ; input . skip (offset . 0) ? ; ArangeHeader :: parse (& mut input , offset) } }
    };
}

impl_361!();