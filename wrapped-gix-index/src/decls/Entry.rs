macro_rules! Entry {
    () => {
        # [doc = " An entry in the index, identifying a non-tree item on disk."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct Entry { # [doc = " The filesystem stat information for the file on disk."] pub stat : entry :: Stat , # [doc = " The object id for this entry's ODB representation (assuming it's up-to-date with it)."] pub id : gix_hash :: ObjectId , # [doc = " Additional flags for use in algorithms and for efficiently storing stage information."] pub flags : entry :: Flags , # [doc = " The kind of item this entry represents - it's not all blobs in the index anymore."] pub mode : entry :: Mode , # [doc = " The range to lookup in the path backing to obtain the entry path relative to the repository."] # [doc = " This costs additional memory but is probably worth it given that paths can stay in one big allocation."] path : Range < usize > , }
    };
}

Entry!()