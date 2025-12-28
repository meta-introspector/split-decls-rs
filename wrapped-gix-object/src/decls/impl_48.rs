macro_rules! deps {
    () => {
        ExtraHeaders!();
        Commit!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl Commit { # [doc = " Returns a convenient iterator over all extra headers."] pub fn extra_headers (& self) -> ExtraHeaders < impl Iterator < Item = (& BStr , & BStr) > > { ExtraHeaders :: new (self . extra_headers . iter () . map (| (k , v) | (k . as_bstr () , v . as_bstr ()))) } }
    };
}

impl_48!();