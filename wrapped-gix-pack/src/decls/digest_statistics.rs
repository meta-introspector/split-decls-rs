macro_rules! deps {
    () => {
        Entry!();
        Tree!();
        Outcome!();
        Kind!();
        Statistics!();
    };
}

macro_rules! digest_statistics {
    () => {
        deps!();
        fn digest_statistics (traverse :: Outcome { roots , children } : traverse :: Outcome < Entry >) -> index :: traverse :: Statistics { let mut res = index :: traverse :: Statistics :: default () ; let average = & mut res . average ; for item in roots . iter () . chain (children . iter ()) { res . total_compressed_entries_size += item . data . compressed_size ; res . total_decompressed_entries_size += item . data . decompressed_size ; res . total_object_size += item . data . object_size ; * res . objects_per_chain_length . entry (u32 :: from (item . data . level)) . or_insert (0) += 1 ; average . decompressed_size += item . data . decompressed_size ; average . compressed_size += item . data . compressed_size as usize ; average . object_size += item . data . object_size ; average . num_deltas += u32 :: from (item . data . level) ; use gix_object :: Kind :: * ; match item . data . object_kind { Blob => res . num_blobs += 1 , Tree => res . num_trees += 1 , Tag => res . num_tags += 1 , Commit => res . num_commits += 1 , } } let num_nodes = roots . len () + children . len () ; average . decompressed_size /= num_nodes as u64 ; average . compressed_size /= num_nodes ; average . object_size /= num_nodes as u64 ; average . num_deltas /= num_nodes as u32 ; res }
    };
}

digest_statistics!();