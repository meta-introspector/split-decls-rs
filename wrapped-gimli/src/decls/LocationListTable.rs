macro_rules! deps {
    () => {
        BaseId!();
        LocationList!();
        FnvIndexSet!();
    };
}

macro_rules! LocationListTable {
    () => {
        deps!();
        # [doc = " A table of location lists that will be stored in a `.debug_loc` or `.debug_loclists` section."] # [derive (Debug , Default)] pub struct LocationListTable { base_id : BaseId , locations : FnvIndexSet < LocationList > , }
    };
}

LocationListTable!();