macro_rules! deps {
    () => {
        Table!();
        ResourceDirectoryTable!();
        ImageResourceDataEntry!();
    };
}

macro_rules! ResourceDirectoryEntryData {
    () => {
        deps!();
        # [doc = " Data associated with a resource directory entry."] # [derive (Debug , Clone)] pub enum ResourceDirectoryEntryData < 'data > { # [doc = " A subtable entry."] Table (ResourceDirectoryTable < 'data >) , # [doc = " A resource data entry."] Data (& 'data pe :: ImageResourceDataEntry) , }
    };
}

ResourceDirectoryEntryData!();