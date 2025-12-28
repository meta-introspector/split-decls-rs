macro_rules! deps {
    () => {
        ToNormalPathComponents!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl ToNormalPathComponents for & Path { fn to_normal_path_components (& self) -> impl Iterator < Item = Result < & OsStr , to_normal_path_components :: Error > > { self . components () . map (| c | component_to_os_str (c , self)) } }
    };
}

impl_51!()