macro_rules! DwoUnit {
    () => {
        # [doc = " A DWO unit has its own DWARF sections."] struct DwoUnit < R : gimli :: Reader > { sections : Arc < gimli :: Dwarf < R > > , dw_unit : gimli :: Unit < R > , }
    };
}

DwoUnit!()