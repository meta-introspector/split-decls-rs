macro_rules! deps {
    () => {
        Ordering!();
        Store!();
        Error!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl Store { # [doc = " Check all loose objects for their integrity checking their hash matches the actual data and by decoding them fully."] pub fn verify_integrity (& self , progress : & mut dyn DynNestedProgress , should_interrupt : & AtomicBool ,) -> Result < integrity :: Statistics , integrity :: Error > { use gix_object :: Write ; let mut buf = Vec :: new () ; let sink = crate :: sink (self . object_hash) ; let mut num_objects = 0 ; let start = Instant :: now () ; let mut progress = progress . add_child_with_id ("Validating" . into () , integrity :: ProgressId :: LooseObjects . into ()) ; progress . init (None , gix_features :: progress :: count ("loose objects")) ; for id in self . iter () . filter_map (Result :: ok) { let object = self . try_find (& id , & mut buf) . map_err (| _ | integrity :: Error :: Retry) ? . ok_or (integrity :: Error :: Retry) ? ; sink . write_buf (object . kind , object . data) . map_err (| err | integrity :: Error :: ObjectHasher { source : * err . downcast () . expect ("sink can only fail in hasher") , kind : object . kind , expected : id , }) ? . verify (& id) . map_err (| err | integrity :: Error :: ObjectEncodeMismatch { source : err , kind : object . kind , }) ? ; object . decode () . map_err (| err | integrity :: Error :: ObjectDecode { source : err , kind : object . kind , id , }) ? ; progress . inc () ; num_objects += 1 ; if should_interrupt . load (Ordering :: SeqCst) { return Err (integrity :: Error :: Interrupted) ; } } progress . show_throughput (start) ; Ok (integrity :: Statistics { num_objects }) } }
    };
}

impl_114!();