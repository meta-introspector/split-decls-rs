macro_rules! deps {
    () => {
        Error!();
        Extensions!();
        Signature!();
        Options!();
        State!();
        Version!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl State { # [doc = " Serialize this instance to `out` with [`options`][Options]."] pub fn write_to (& self , out : impl std :: io :: Write , Options { extensions , skip_hash : _ , } : Options ,) -> Result < Version , gix_hash :: io :: Error > { let _span = gix_features :: trace :: detail ! ("gix_index::State::write()") ; let version = self . detect_required_version () ; let mut write = CountBytes :: new (out) ; let num_entries : u32 = self . entries () . len () . try_into () . expect ("definitely not 4billion entries") ; let removed_entries : u32 = self . entries () . iter () . filter (| e | e . flags . contains (entry :: Flags :: REMOVE)) . count () . try_into () . expect ("definitely not too many entries") ; let offset_to_entries = header (& mut write , version , num_entries - removed_entries) ? ; let offset_to_extensions = entries (& mut write , self , offset_to_entries) ? ; let (extension_toc , out) = self . write_extensions (write , offset_to_extensions , extensions) ? ; if num_entries > 0 && extensions . should_write (extension :: end_of_index_entry :: SIGNATURE) . is_some () && ! extension_toc . is_empty () { extension :: end_of_index_entry :: write_to (out , self . object_hash , offset_to_extensions , extension_toc) ? ; } Ok (version) } fn write_extensions < T > (& self , mut write : CountBytes < T > , offset_to_extensions : u32 , extensions : Extensions ,) -> std :: io :: Result < (Vec < (extension :: Signature , u32) > , T) > where T : std :: io :: Write , { type WriteExtFn < 'a > = & 'a dyn Fn (& mut dyn std :: io :: Write) -> Option < std :: io :: Result < extension :: Signature > > ; let extensions : & [WriteExtFn < '_ >] = & [& | write | { extensions . should_write (extension :: tree :: SIGNATURE) . and_then (| signature | self . tree () . map (| tree | tree . write_to (write) . map (| _ | signature))) } , & | write | { self . is_sparse () . then (| | extension :: sparse :: write_to (write) . map (| _ | extension :: sparse :: SIGNATURE)) } ,] ; let mut offset_to_previous_ext = offset_to_extensions ; let mut out = Vec :: with_capacity (5) ; for write_ext in extensions { if let Some (signature) = write_ext (& mut write) . transpose () ? { let offset_past_ext = write . count ; let ext_size = offset_past_ext - offset_to_previous_ext - (extension :: MIN_SIZE as u32) ; offset_to_previous_ext = offset_past_ext ; out . push ((signature , ext_size)) ; } } Ok ((out , write . inner)) } }
    };
}

impl_148!();