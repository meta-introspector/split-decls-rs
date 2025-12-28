macro_rules! deps {
    () => {
        Error!();
        Id!();
        Tag!();
        Note!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < 'repo > Tag < 'repo > { # [doc = " Decode the entire tag object and return it for accessing all tag information."] # [doc = ""] # [doc = " This never allocates."] # [doc = ""] # [doc = " Note that the returned commit object does make lookup easy and should be"] # [doc = " used for successive calls to string-ish information to avoid decoding the object"] # [doc = " more than once."] pub fn decode (& self) -> Result < gix_object :: TagRef < '_ > , gix_object :: decode :: Error > { gix_object :: TagRef :: from_bytes (& self . data) } # [doc = " Decode this tag partially and return the id of its target."] pub fn target_id (& self) -> Result < crate :: Id < 'repo > , gix_object :: decode :: Error > { gix_object :: TagRefIter :: from_bytes (& self . data) . target_id () . map (| id | id . attach (self . repo)) } # [doc = " Decode this tag partially and return the tagger, if the field exists."] pub fn tagger (& self) -> Result < Option < gix_actor :: SignatureRef < '_ > > , gix_object :: decode :: Error > { gix_object :: TagRefIter :: from_bytes (& self . data) . tagger () } }
    };
}

impl_181!();