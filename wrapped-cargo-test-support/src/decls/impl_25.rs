macro_rules! deps {
    () => {
        InMemoryDir!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < P , D > From < std :: collections :: BTreeMap < P , D > > for InMemoryDir where P : Into < PathBuf > , D : IntoData , { fn from (files : std :: collections :: BTreeMap < P , D >) -> Self { let files = files . into_iter () . map (| (p , d) | (p . into () , d . into_data ())) . collect () ; Self { files } } }
    };
}

impl_25!()