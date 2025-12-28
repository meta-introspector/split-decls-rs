macro_rules! impl_304 {
    () => {
        impl hyper_headers { pub (super) fn get_or_default (ext : & mut http :: Extensions) -> & mut hyper_headers { if let None = ext . get_mut :: < hyper_headers > () { ext . insert (hyper_headers :: default ()) ; } ext . get_mut :: < hyper_headers > () . unwrap () } }
    };
}

impl_304!()