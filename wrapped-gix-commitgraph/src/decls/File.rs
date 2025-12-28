macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! File {
    () => {
        deps!();
        # [doc = " A single commit-graph file."] # [doc = ""] # [doc = " All operations on a `File` are local to that graph file. Since a commit graph can span multiple"] # [doc = " files, all interesting graph operations belong on [`Graph`]."] pub struct File { base_graph_count : u8 , base_graphs_list_offset : Option < usize > , commit_data_offset : usize , data : memmap2 :: Mmap , extra_edges_list_range : Option < std :: ops :: Range < usize > > , fan : [u32 ; file :: FAN_LEN] , oid_lookup_offset : usize , path : std :: path :: PathBuf , hash_len : usize , object_hash : gix_hash :: Kind , }
    };
}

File!();