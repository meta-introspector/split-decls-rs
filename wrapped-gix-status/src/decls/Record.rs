macro_rules! deps {
    () => {
        EntryStatus!();
        Entry!();
    };
}

macro_rules! Record {
    () => {
        deps!();
        # [doc = " A record of a change."] # [doc = ""] # [doc = " It's created either if there is a conflict or a change, or both."] # [derive (Debug , Clone)] pub struct Record < 'index , T , U > { # [doc = " The index entry that is changed."] pub entry : & 'index index :: Entry , # [doc = " The index of the `entry` relative to all entries in the input index."] pub entry_index : usize , # [doc = " The path to the entry."] pub relative_path : & 'index BStr , # [doc = " The status information itself."] pub status : EntryStatus < T , U > , }
    };
}

Record!()