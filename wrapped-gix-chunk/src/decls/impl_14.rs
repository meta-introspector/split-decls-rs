macro_rules! deps {
    () => {
        Index!();
        Entry!();
        Id!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [doc = " Writing"] impl Index { # [doc = " Create a new index whose sole purpose is to be receiving chunks using [`plan_chunk()`][Index::plan_chunk()] and to be written to"] # [doc = " an output using [`into_write()`][Index::into_write()]"] pub fn for_writing () -> Self { Index { will_write : true , chunks : Vec :: new () , } } # [doc = " Plan to write a new chunk as part of the index when [`into_write()`][Index::into_write()] is called."] pub fn plan_chunk (& mut self , chunk : crate :: Id , exact_size_on_disk : u64) { assert ! (self . will_write , "BUG: create the index with `for_writing()`") ; assert ! (! self . chunks . iter () . any (| e | e . kind == chunk) , "BUG: must not add chunk of same kind twice: {:?}" , std :: str :: from_utf8 (& chunk)) ; self . chunks . push (Entry { kind : chunk , offset : 0 .. exact_size_on_disk , }) ; } # [doc = " Return the total size of all planned chunks thus far."] pub fn planned_storage_size (& self) -> u64 { assert ! (self . will_write , "BUG: create the index with `for_writing()`") ; self . chunks . iter () . map (| e | e . offset . end) . sum () } # [doc = " Return the amount of chunks we currently know."] pub fn num_chunks (& self) -> usize { self . chunks . len () } # [doc = " After [planning all chunks][Index::plan_chunk()] call this method with the destination to write the chunks to."] # [doc = " Use the [Chunk] writer to write each chunk in order."] # [doc = " `current_offset` is the byte position at which `out` will continue writing."] pub fn into_write < W > (self , mut out : W , current_offset : usize) -> std :: io :: Result < Chunk < W > > where W : std :: io :: Write , { assert ! (self . will_write , "BUG: create the index with `for_writing()`, cannot write decoded indices") ; let mut current_offset = (current_offset + Self :: size_for_entries (self . num_chunks ())) as u64 ; for entry in & self . chunks { out . write_all (& entry . kind) ? ; out . write_all (& current_offset . to_be_bytes ()) ? ; current_offset += entry . offset . end ; } out . write_all (& 0u32 . to_be_bytes ()) ? ; out . write_all (& current_offset . to_be_bytes ()) ? ; Ok (Chunk :: new (out , self . chunks . into ())) } }
    };
}

impl_14!()