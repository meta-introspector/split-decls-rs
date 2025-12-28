macro_rules! deps {
    () => {
        Store!();
        Error!();
        IndexState!();
        IndexAndPacks!();
        Record!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl Store { # [doc = " Return information about all files known to us as well as their loading state."] # [doc = ""] # [doc = " Note that this call is expensive as it gathers additional information about loose object databases."] # [doc = " Note that it may change as we collect information due to the highly volatile nature of the"] # [doc = " implementation. The likelihood of actual changes is low though as these still depend on something"] # [doc = " changing on disk and somebody reading at the same time."] pub fn structure (& self) -> Result < Vec < Record > , load_index :: Error > { let _span = gix_features :: trace :: detail ! ("gix_odb::Store::structure()") ; let index = self . index . load () ; if ! index . is_initialized () { self . consolidate_with_disk_state (true , false) ? ; } let index = self . index . load () ; let mut res : Vec < _ > = index . loose_dbs . iter () . map (| db | Record :: LooseObjectDatabase { objects_directory : db . path . clone () , num_objects : db . iter () . count () , }) . collect () ; for slot in index . slot_indices . iter () . map (| idx | & self . files [* idx]) { let files = slot . files . load () ; let record = match & * * files { Some (index) => { let state = if index . is_disposable () { IndexState :: Disposable } else if index . index_is_loaded () { IndexState :: Loaded } else { IndexState :: Unloaded } ; match index { IndexAndPacks :: Index (b) => Record :: Index { path : b . index . path () . into () , state , } , IndexAndPacks :: MultiIndex (b) => Record :: MultiIndex { path : b . multi_index . path () . into () , state , } , } } None => Record :: Empty , } ; res . push (record) ; } Ok (res) } # [doc = " Provide a list of all `objects` directories of `alternate` object database paths."] # [doc = " This list might be empty if there are no alternates."] # [doc = ""] # [doc = " Read more about alternates in the documentation of the [`resolve`][crate::alternate::resolve()] function."] pub fn alternate_db_paths (& self) -> Result < Vec < PathBuf > , load_index :: Error > { let index = self . index . load () ; if ! index . is_initialized () { self . consolidate_with_disk_state (true , false) ? ; } let index = self . index . load () ; Ok (index . loose_dbs . iter () . skip (1 ,) . map (| db | db . path . clone ()) . collect ()) } }
    };
}

impl_98!();