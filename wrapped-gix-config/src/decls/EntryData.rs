macro_rules! deps {
    () => {
        SectionId!();
    };
}

macro_rules! EntryData {
    () => {
        deps!();
        # [doc = " Internal data structure for [`MutableMultiValue`]"] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub (crate) struct EntryData { pub (crate) section_id : SectionId , pub (crate) offset_index : usize , }
    };
}

EntryData!()