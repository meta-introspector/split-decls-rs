macro_rules! deps {
    () => {
        Kind!();
        Version!();
    };
}

macro_rules! File {
    () => {
        deps!();
        # [doc = " A representation of an index file for multiple packs at the same time, typically stored in a file"] # [doc = " named 'multi-pack-index'."] pub struct File { data : Mmap , path : std :: path :: PathBuf , version : Version , hash_len : usize , object_hash : gix_hash :: Kind , # [doc = " The amount of pack files contained within"] num_indices : u32 , num_objects : u32 , fan : [u32 ; 256] , index_names : Vec < PathBuf > , lookup_ofs : usize , offsets_ofs : usize , large_offsets_ofs : Option < usize > , }
    };
}

File!()