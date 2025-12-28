macro_rules! deps {
    () => {
        Reader!();
        DebuggingInformationEntry!();
        RangeIterInner!();
    };
}

macro_rules! RangeIter {
    () => {
        deps!();
        # [doc = " An iterator for the address ranges of a `DebuggingInformationEntry`."] # [doc = ""] # [doc = " Returned by `Dwarf::die_ranges` and `Dwarf::unit_ranges`."] # [derive (Debug)] pub struct RangeIter < R : Reader > (RangeIterInner < R >) ;
    };
}

RangeIter!()