macro_rules! deps {
    () => {
        MetadataCollection!();
        Metadata!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        # [doc = " Access"] impl MetadataCollection { # [doc = " Return an iterator over the contents of the map in an easy-to-consume form."] pub fn iter (& self) -> impl Iterator < Item = (& str , & Metadata) > { self . name_to_meta . iter () . map (| (k , v) | (k . as_str () , v)) } }
    };
}

impl_40!();