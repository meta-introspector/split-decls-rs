macro_rules! deps {
    () => {
        Default!();
        Platform!();
        Item!();
        Repository!();
    };
}

macro_rules! impl_863 {
    () => {
        deps!();
        impl < 'repo > Platform < 'repo > { pub (crate) fn new (tips : impl IntoIterator < Item = impl Into < ObjectId > > , repo : & 'repo Repository) -> Self { revision :: walk :: Platform { repo , tips : tips . into_iter () . map (Into :: into) . collect () , hidden : Vec :: new () , sorting : Default :: default () , parents : Default :: default () , use_commit_graph : None , commit_graph : None , boundary : Vec :: new () , } } }
    };
}

impl_863!()