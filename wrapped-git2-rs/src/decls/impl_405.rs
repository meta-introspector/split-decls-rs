macro_rules! deps {
    () => {
        IndexerProgress!();
        OdbPackwriterCb!();
        Odb!();
        Error!();
        Indexer!();
        Binding!();
        Progress!();
    };
}

macro_rules! impl_405 {
    () => {
        deps!();
        impl < 'a > Indexer < 'a > { # [doc = " Create a new indexer"] # [doc = ""] # [doc = " The [`Odb`] is used to resolve base objects when fixing thin packs. It"] # [doc = " can be `None` if no thin pack is expected, in which case missing bases"] # [doc = " will result in an error."] # [doc = ""] # [doc = " `path` is the directory where the packfile should be stored."] # [doc = ""] # [doc = " `mode` is the permissions to use for the output files, use `0` for defaults."] # [doc = ""] # [doc = " If `verify` is `false`, the indexer will bypass object connectivity checks."] pub fn new (odb : Option < & Odb < 'a > > , path : & Path , mode : u32 , verify : bool) -> Result < Self , Error > { crate :: init () ; let path = path . into_c_string () ? ; let odb = odb . map (Binding :: raw) . unwrap_or_else (ptr :: null_mut) ; let mut out = ptr :: null_mut () ; let progress_cb : raw :: git_indexer_progress_cb = Some (write_pack_progress_cb) ; let progress_payload = Box :: new (OdbPackwriterCb { cb : None }) ; let progress_payload_ptr = Box :: into_raw (progress_payload) ; unsafe { let mut opts = mem :: zeroed () ; try_call ! (raw :: git_indexer_options_init (& mut opts , raw :: GIT_INDEXER_OPTIONS_VERSION)) ; opts . progress_cb = progress_cb ; opts . progress_cb_payload = progress_payload_ptr as * mut c_void ; opts . verify = verify . into () ; try_call ! (raw :: git_indexer_new (& mut out , path , mode , odb , & mut opts)) ; } Ok (Self { raw : out , progress : Default :: default () , progress_payload_ptr , }) } # [doc = " Finalize the pack and index"] # [doc = ""] # [doc = " Resolves any pending deltas and writes out the index file. The returned"] # [doc = " string is the hexadecimal checksum of the packfile, which is also used"] # [doc = " to name the pack and index files (`pack-<checksum>.pack` and"] # [doc = " `pack-<checksum>.idx` respectively)."] pub fn commit (mut self) -> Result < String , Error > { unsafe { try_call ! (raw :: git_indexer_commit (self . raw , & mut self . progress)) ; let name = CStr :: from_ptr (raw :: git_indexer_name (self . raw)) ; Ok (name . to_str () . expect ("pack name not utf8") . to_owned ()) } } # [doc = " The callback through which progress is monitored. Be aware that this is"] # [doc = " called inline, so performance may be affected."] pub fn progress < F > (& mut self , cb : F) -> & mut Self where F : FnMut (Progress < '_ >) -> bool + 'a , { let progress_payload = unsafe { & mut * (self . progress_payload_ptr as * mut OdbPackwriterCb < '_ >) } ; progress_payload . cb = Some (Box :: new (cb) as Box < IndexerProgress < 'a > >) ; self } }
    };
}

impl_405!()