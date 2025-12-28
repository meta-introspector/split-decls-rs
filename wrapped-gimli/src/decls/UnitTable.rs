macro_rules! deps {
    () => {
        BaseId!();
        Unit!();
    };
}

macro_rules! UnitTable {
    () => {
        deps!();
        # [doc = " A table of units that will be stored in the `.debug_info` section."] # [derive (Debug , Default)] pub struct UnitTable { base_id : BaseId , units : Vec < Unit > , }
    };
}

UnitTable!()