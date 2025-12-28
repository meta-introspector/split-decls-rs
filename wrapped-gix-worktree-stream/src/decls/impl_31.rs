macro_rules! deps {
    () => {
        Error!();
        AdditionalEntry!();
        Stream!();
        Source!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [doc = " Entries"] impl Stream { # [doc = " Add `entry` to the list of entries to be returned in calls to [`Self::next_entry()`]."] # [doc = ""] # [doc = " The entry will be returned after the one contained in the tree, in order of addition."] # [doc = " # Panics"] # [doc = " If called after the first call to [`Self::next_entry()`]."] pub fn add_entry (& mut self , entry : AdditionalEntry) -> & mut Self { self . extra_entries . as_ref () . expect ("BUG: must not add entries after the start of entries traversal") . send (entry) . expect ("Failure is impossible as thread blocks on the receiving end") ; self } # [doc = " Add the item at `path` as entry to this stream, which is expected to be under `root`."] # [doc = ""] # [doc = " Note that the created entries will always have a null SHA1, and that we access this path"] # [doc = " to determine its type, and will access it again when it is requested."] pub fn add_entry_from_path (& mut self , root : & Path , path : & Path) -> std :: io :: Result < & mut Self > { let rela_path = path . strip_prefix (root) . map_err (std :: io :: Error :: other) ? ; let meta = path . symlink_metadata () ? ; let relative_path = gix_path :: to_unix_separators_on_windows (gix_path :: into_bstr (rela_path)) . into_owned () ; let id = gix_hash :: ObjectId :: null (gix_hash :: Kind :: Sha1) ; let entry = if meta . is_symlink () { let content = std :: fs :: read_link (path) ? ; let content = gix_path :: into_bstr (content) . into_owned () ; AdditionalEntry { id , mode : gix_object :: tree :: EntryKind :: Link . into () , relative_path , source : entry :: Source :: Memory (content . into ()) , } } else if meta . is_dir () { AdditionalEntry { id , mode : gix_object :: tree :: EntryKind :: Tree . into () , relative_path , source : entry :: Source :: Null , } } else { let mode = if gix_fs :: is_executable (& meta) { gix_object :: tree :: EntryKind :: BlobExecutable } else { gix_object :: tree :: EntryKind :: Blob } . into () ; AdditionalEntry { id , mode , relative_path , source : entry :: Source :: Path (path . to_owned ()) , } } ; Ok (self . add_entry (entry)) } }
    };
}

impl_31!();