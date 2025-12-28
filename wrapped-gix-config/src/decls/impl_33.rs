macro_rules! deps {
    () => {
        Body!();
        Section!();
        Error!();
        Options!();
        Metadata!();
        File!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a > File < 'a > { # [doc = " Return an empty `File` with the given `meta`-data to be attached to all new sections."] pub fn new (meta : impl Into < OwnShared < Metadata > >) -> Self { Self { frontmatter_events : Default :: default () , frontmatter_post_section : Default :: default () , section_lookup_tree : Default :: default () , sections : Default :: default () , section_id_counter : 0 , section_order : Default :: default () , meta : meta . into () , } } # [doc = " Instantiate a new `File` from given `input`, associating each section and their values with"] # [doc = " `meta`-data, while respecting `options`."] pub fn from_bytes_no_includes (input : & 'a [u8] , meta : impl Into < OwnShared < Metadata > > , options : Options < '_ > ,) -> Result < Self , Error > { let meta = meta . into () ; Ok (Self :: from_parse_events_no_includes (parse :: Events :: from_bytes (input , options . to_event_filter ()) ? , meta ,)) } # [doc = " Instantiate a new `File` from given `events`, associating each section and their values with"] # [doc = " `meta`-data."] pub fn from_parse_events_no_includes (parse :: Events { frontmatter , sections } : parse :: Events < 'a > , meta : impl Into < OwnShared < Metadata > > ,) -> Self { let meta = meta . into () ; let mut this = File :: new (OwnShared :: clone (& meta)) ; this . frontmatter_events = frontmatter ; this . sections . reserve (sections . len ()) ; this . section_order . reserve (sections . len ()) ; for section in sections { this . push_section_internal (crate :: file :: Section { header : section . header , body : section :: Body (section . events) , meta : OwnShared :: clone (& meta) , id : Default :: default () , }) ; } this } }
    };
}

impl_33!()