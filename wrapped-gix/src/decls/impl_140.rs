macro_rules! deps {
    () => {
        Error!();
        Id!();
        Object!();
        Note!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        # [doc = " An [object id][ObjectId] infused with a [`Repository`][crate::Repository]."] impl < 'repo > Id < 'repo > { # [doc = " Find the [`Object`] associated with this object id, and consider it an error if it doesn't exist."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " There can only be one `ObjectRef` per `Easy`. To increase that limit, clone the `Easy`."] pub fn object (& self) -> Result < Object < 'repo > , find :: existing :: Error > { self . repo . find_object (self . inner) } # [doc = " Find the [`header`][gix_odb::find::Header] associated with this object id, or an error if it doesn't exist."] # [doc = ""] # [doc = " Use this method if there is no interest in the contents of the object, which generally is much faster to obtain."] pub fn header (& self) -> Result < gix_odb :: find :: Header , find :: existing :: Error > { self . repo . find_header (self . inner) } # [doc = " Try to find the [`Object`] associated with this object id, and return `None` if it's not available locally."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " There can only be one `ObjectRef` per `Easy`. To increase that limit, clone the `Easy`."] pub fn try_object (& self) -> Result < Option < Object < 'repo > > , find :: Error > { self . repo . try_find_object (self . inner) } # [doc = " Find the [`header`][gix_odb::find::Header] associated with this object id, or return `None` if it doesn't exist."] # [doc = ""] # [doc = " Use this method if there is no interest in the contents of the object, which generally is much faster to obtain."] pub fn try_header (& self) -> Result < Option < gix_odb :: find :: Header > , find :: Error > { self . repo . try_find_header (self . inner) } # [doc = " Turn this object id into a shortened id with a length in hex as configured by `core.abbrev`."] pub fn shorten (& self) -> Result < gix_hash :: Prefix , shorten :: Error > { let hex_len = self . repo . config . hex_len . map_or_else (| | self . repo . objects . packed_object_count () . map (calculate_auto_hex_len) , Ok ,) ? ; let prefix = gix_odb :: store :: prefix :: disambiguate :: Candidate :: new (self . inner , hex_len) . expect ("BUG: internal hex-len must always be valid") ; self . repo . objects . disambiguate_prefix (prefix) ? . ok_or (shorten :: Error :: NotFound { oid : self . inner }) } # [doc = " Turn this object id into a shortened id with a length in hex as configured by `core.abbrev`, or default"] # [doc = " to a prefix which equals our id in the unlikely error case."] pub fn shorten_or_id (& self) -> gix_hash :: Prefix { self . shorten () . unwrap_or_else (| _ | self . inner . into ()) } }
    };
}

impl_140!()