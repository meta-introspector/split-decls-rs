macro_rules! ReferenceStyle {
    () => {
        # [derive (Debug , PartialEq)] pub enum ReferenceStyle { Full , Flat , SkipRoot , }
    };
}

ReferenceStyle!();