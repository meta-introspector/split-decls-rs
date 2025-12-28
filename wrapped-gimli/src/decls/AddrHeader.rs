macro_rules! deps {
    () => {
        DebugAddrOffset!();
        Reader!();
        Encoding!();
        ReaderOffset!();
    };
}

macro_rules! AddrHeader {
    () => {
        deps!();
        # [doc = " A header for a set of entries in the `.debug_addr` section."] # [doc = ""] # [doc = " These entries all belong to a single unit."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct AddrHeader < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { offset : DebugAddrOffset < Offset > , encoding : Encoding , length : Offset , entries : R , }
    };
}

AddrHeader!()