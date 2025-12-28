macro_rules! deps {
    () => {
        InMemoryDir!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < P , D > FromIterator < (P , D) > for InMemoryDir where P : Into < std :: path :: PathBuf > , D : IntoData , { fn from_iter < I : IntoIterator < Item = (P , D) > > (files : I) -> Self { let files = files . into_iter () . map (| (p , d) | (p . into () , d . into_data ())) . collect () ; Self { files } } }
    };
}

impl_22!()