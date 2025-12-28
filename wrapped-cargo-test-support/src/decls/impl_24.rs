macro_rules! deps {
    () => {
        InMemoryDir!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < P , D > From < std :: collections :: HashMap < P , D > > for InMemoryDir where P : Into < PathBuf > , D : IntoData , { fn from (files : std :: collections :: HashMap < P , D >) -> Self { let files = files . into_iter () . map (| (p , d) | (p . into () , d . into_data ())) . collect () ; Self { files } } }
    };
}

impl_24!();