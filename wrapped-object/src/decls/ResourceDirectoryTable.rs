macro_rules! deps {
    () => {
        ImageResourceDirectory!();
        ImageResourceDirectoryEntry!();
    };
}

macro_rules! ResourceDirectoryTable {
    () => {
        deps!();
        # [doc = " A table of resource entries."] # [derive (Debug , Clone)] pub struct ResourceDirectoryTable < 'data > { # [doc = " The table header."] pub header : & 'data pe :: ImageResourceDirectory , # [doc = " The table entries."] pub entries : & 'data [pe :: ImageResourceDirectoryEntry] , }
    };
}

ResourceDirectoryTable!();