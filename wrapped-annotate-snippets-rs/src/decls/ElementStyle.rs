macro_rules! deps {
    () => {
        Level!();
        LevelInner!();
    };
}

macro_rules! ElementStyle {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialOrd , Ord , PartialEq , Eq)] pub (crate) enum ElementStyle { MainHeaderMsg , HeaderMsg , LineAndColumn , LineNumber , Quotation , UnderlinePrimary , UnderlineSecondary , LabelPrimary , LabelSecondary , NoStyle , Level (LevelInner) , Addition , Removal , }
    };
}

ElementStyle!();