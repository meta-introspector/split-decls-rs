macro_rules! deps {
    () => {
        RangeList!();
        BaseId!();
        FnvIndexSet!();
    };
}

macro_rules! RangeListTable {
    () => {
        deps!();
        # [doc = " A table of range lists that will be stored in a `.debug_ranges` or `.debug_rnglists` section."] # [derive (Debug , Default)] pub struct RangeListTable { base_id : BaseId , ranges : FnvIndexSet < RangeList > , }
    };
}

RangeListTable!();