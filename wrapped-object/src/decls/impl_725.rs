macro_rules! deps {
    () => {
        ImageResourceDataEntry!();
        Table!();
        ResourceDirectoryEntryData!();
        ResourceDirectoryTable!();
    };
}

macro_rules! impl_725 {
    () => {
        deps!();
        impl < 'data > ResourceDirectoryEntryData < 'data > { # [doc = " Converts to an option of table."] # [doc = ""] # [doc = " Helper for iterator filtering."] pub fn table (self) -> Option < ResourceDirectoryTable < 'data > > { match self { Self :: Table (dir) => Some (dir) , _ => None , } } # [doc = " Converts to an option of data entry."] # [doc = ""] # [doc = " Helper for iterator filtering."] pub fn data (self) -> Option < & 'data pe :: ImageResourceDataEntry > { match self { Self :: Data (rsc) => Some (rsc) , _ => None , } } }
    };
}

impl_725!();