macro_rules! deps {
    () => {
        ToNormalPathComponents!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl ToNormalPathComponents for & BString { fn to_normal_path_components (& self) -> impl Iterator < Item = Result < & OsStr , to_normal_path_components :: Error > > { self . split (| b | * b == b'/') . filter_map (| c | bytes_component_to_os_str (c , self . as_bstr ())) } }
    };
}

impl_56!();