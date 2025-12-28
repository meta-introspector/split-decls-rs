macro_rules! deps {
    () => {
        Tree!();
        Statistics!();
        Outcome!();
        Kind!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl Default for Statistics { fn default () -> Self { Statistics { average : crate :: data :: decode :: entry :: Outcome :: default_from_kind (gix_object :: Kind :: Tree) , objects_per_chain_length : Default :: default () , total_compressed_entries_size : 0 , total_decompressed_entries_size : 0 , total_object_size : 0 , pack_size : 0 , num_blobs : 0 , num_commits : 0 , num_trees : 0 , num_tags : 0 , } } }
    };
}

impl_254!()