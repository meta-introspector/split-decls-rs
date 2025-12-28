macro_rules! deps {
    () => {
        InMemoryDir!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < const N : usize , P , D > From < [(P , D) ; N] > for InMemoryDir where P : Into < PathBuf > , D : IntoData , { fn from (files : [(P , D) ; N]) -> Self { let files = files . into_iter () . map (| (p , d) | (p . into () , d . into_data ())) . collect () ; Self { files } } }
    };
}

impl_23!();