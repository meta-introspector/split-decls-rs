macro_rules! Item {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] pub enum Item { MacroDef (Box < MacroDefId >) , Other , }
    };
}

Item!();