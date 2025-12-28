macro_rules! deps {
    () => {
        AutoRemove!();
        Mode!();
        ContainingDirectory!();
        ForksafeTempfile!();
        Handle!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [doc = " Utilities"] impl Handle < () > { fn at_path (path : & Path , directory : ContainingDirectory , cleanup : AutoRemove , mode : Mode , permissions : Option < std :: fs :: Permissions > ,) -> io :: Result < usize > { let tempfile = { let mut builder = tempfile :: Builder :: new () ; let dot_ext_storage ; match path . file_stem () { Some (stem) => builder . prefix (stem) , None => builder . prefix ("") , } ; if let Some (ext) = path . extension () { dot_ext_storage = format ! (".{}" , ext . to_string_lossy ()) ; builder . suffix (& dot_ext_storage) ; } if let Some (permissions) = permissions { builder . permissions (permissions) ; } let parent_dir = path . parent () . expect ("parent directory is present") ; let parent_dir = directory . resolve (parent_dir) ? ; ForksafeTempfile :: new (builder . rand_bytes (0) . tempfile_in (parent_dir) ? , cleanup , mode) } ; let id = NEXT_MAP_INDEX . fetch_add (1 , std :: sync :: atomic :: Ordering :: SeqCst) ; expect_none (REGISTRY . insert (id , Some (tempfile))) ; Ok (id) } fn new_writable_inner (containing_directory : & Path , directory : ContainingDirectory , cleanup : AutoRemove , mode : Mode ,) -> io :: Result < usize > { let containing_directory = directory . resolve (containing_directory) ? ; let id = NEXT_MAP_INDEX . fetch_add (1 , std :: sync :: atomic :: Ordering :: SeqCst) ; expect_none (REGISTRY . insert (id , Some (ForksafeTempfile :: new (NamedTempFile :: new_in (containing_directory) ? , cleanup , mode ,)) ,)) ; Ok (id) } }
    };
}

impl_14!()