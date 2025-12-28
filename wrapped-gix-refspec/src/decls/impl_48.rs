macro_rules! deps {
    () => {
        Needle!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a > From < & 'a BStr > for Needle < 'a > { fn from (v : & 'a BStr) -> Self { if let Some (pos) = v . find_byte (b'*') { Needle :: Glob { name : v , asterisk_pos : pos , } } else if v . starts_with (b"refs/") { Needle :: FullName (v) } else if let Ok (id) = gix_hash :: ObjectId :: from_hex (v) { Needle :: Object (id) } else { Needle :: PartialName (v) } } }
    };
}

impl_48!();