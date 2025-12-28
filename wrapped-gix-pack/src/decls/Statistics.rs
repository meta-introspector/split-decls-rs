macro_rules! deps {
    () => {
        Outcome!();
    };
}

macro_rules! Statistics {
    () => {
        deps!();
        # [doc = " Statistics regarding object encountered during execution of the [`traverse()`][crate::index::File::traverse()] method."] # [derive (Debug , PartialEq , Eq , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Statistics { # [doc = " The average over all decoded objects"] pub average : crate :: data :: decode :: entry :: Outcome , # [doc = " A mapping of the length of the chain to the amount of objects at that length."] # [doc = ""] # [doc = " A length of 0 indicates full objects, and everything more than that uses the given number"] # [doc = " of delta objects."] pub objects_per_chain_length : BTreeMap < u32 , u32 > , # [doc = " The amount of bytes in all compressed streams, one per entry"] pub total_compressed_entries_size : u64 , # [doc = " The amount of bytes in all decompressed streams, one per entry"] pub total_decompressed_entries_size : u64 , # [doc = " The amount of bytes occupied by all undeltified, decompressed objects"] pub total_object_size : u64 , # [doc = " The amount of bytes occupied by the pack itself, in bytes"] pub pack_size : u64 , # [doc = " The amount of objects encountered that where commits"] pub num_commits : u32 , # [doc = " The amount of objects encountered that where trees"] pub num_trees : u32 , # [doc = " The amount of objects encountered that where tags"] pub num_tags : u32 , # [doc = " The amount of objects encountered that where blobs"] pub num_blobs : u32 , }
    };
}

Statistics!()