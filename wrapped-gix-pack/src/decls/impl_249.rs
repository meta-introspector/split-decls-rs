macro_rules! deps {
    () => {
        Reducer!();
        Outcome!();
        File!();
        Options!();
        DecodeEntry!();
        Entry!();
        ProgressId!();
        Kind!();
        Error!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        # [doc = " Verify and validate the content of the index file"] impl index :: File { # [doc = " Iterate through all _decoded objects_ in the given `pack` and handle them with a `Processor` using a cache to reduce the amount of"] # [doc = " waste while decoding objects."] # [doc = ""] # [doc = " For more details, see the documentation on the [`traverse()`][index::File::traverse()] method."] pub fn traverse_with_lookup < C , Processor , E , F > (& self , mut processor : Processor , pack : & data :: File , progress : & mut dyn DynNestedProgress , should_interrupt : & AtomicBool , Options { thread_limit , check , make_pack_lookup_cache , } : Options < F > ,) -> Result < Outcome , Error < E > > where C : crate :: cache :: DecodeEntry , E : std :: error :: Error + Send + Sync + 'static , Processor : FnMut (gix_object :: Kind , & [u8] , & index :: Entry , & dyn Progress) -> Result < () , E > + Send + Clone , F : Fn () -> C + Send + Clone , { let (verify_result , traversal_result) = parallel :: join ({ let mut pack_progress = progress . add_child_with_id (format ! ("Hash of pack '{}'" , pack . path () . file_name () . expect ("pack has filename") . to_string_lossy ()) , ProgressId :: HashPackDataBytes . into () ,) ; let mut index_progress = progress . add_child_with_id (format ! ("Hash of index '{}'" , self . path . file_name () . expect ("index has filename") . to_string_lossy ()) , ProgressId :: HashPackIndexBytes . into () ,) ; move | | { let res = self . possibly_verify (pack , check , & mut pack_progress , & mut index_progress , should_interrupt) ; if res . is_err () { should_interrupt . store (true , Ordering :: SeqCst) ; } res } } , | | { let index_entries = util :: index_entries_sorted_by_offset_ascending (self , & mut progress . add_child_with_id ("collecting sorted index" . into () , ProgressId :: CollectSortedIndexEntries . into () ,) ,) ; let (chunk_size , thread_limit , available_cores) = parallel :: optimize_chunk_size_and_thread_limit (1000 , Some (index_entries . len ()) , thread_limit , None) ; let there_are_enough_entries_to_process = | | index_entries . len () > chunk_size * available_cores ; let input_chunks = index_entries . chunks (chunk_size) ; let reduce_progress = OwnShared :: new (Mutable :: new ({ let mut p = progress . add_child_with_id ("Traversing" . into () , ProgressId :: DecodedObjects . into ()) ; p . init (Some (self . num_objects () as usize) , progress :: count ("objects")) ; p })) ; let state_per_thread = { let reduce_progress = reduce_progress . clone () ; move | index | { (make_pack_lookup_cache () , Vec :: with_capacity (2048) , zlib :: Inflate :: default () , lock (& reduce_progress) . add_child_with_id (format ! ("thread {index}") , gix_features :: progress :: UNKNOWN) ,) } } ; in_parallel_if (there_are_enough_entries_to_process , input_chunks , thread_limit , state_per_thread , move | entries : & [index :: Entry] , (cache , buf , inflate , progress) | -> Result < Vec < data :: decode :: entry :: Outcome > , Error < _ > > { progress . init (Some (entries . len ()) , gix_features :: progress :: count_with_decimals ("objects" , 2) ,) ; let mut stats = exact_vec (entries . len ()) ; progress . set (0) ; for index_entry in entries . iter () { let result = self . decode_and_process_entry (check , pack , cache , buf , inflate , progress , index_entry , & mut processor ,) ; progress . inc () ; let stat = match result { Err (err @ Error :: PackDecode { .. }) if ! check . fatal_decode_error () => { progress . info (format ! ("Ignoring decode error: {err}")) ; continue ; } res => res , } ? ; stats . push (stat) ; if should_interrupt . load (Ordering :: Relaxed) { break ; } } Ok (stats) } , Reducer :: from_progress (reduce_progress , pack . data_len () , check , should_interrupt) ,) } ,) ; Ok (Outcome { actual_index_checksum : verify_result ? , statistics : traversal_result ? , }) } }
    };
}

impl_249!();