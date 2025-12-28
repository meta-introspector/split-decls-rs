macro_rules! deps {
    () => {
        Metadata!();
        PackageId!();
        Package!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a > std :: ops :: Index < & 'a PackageId > for Metadata { type Output = Package ; fn index (& self , idx : & 'a PackageId) -> & Self :: Output { self . packages . iter () . find (| p | p . id == * idx) . unwrap_or_else (| | panic ! ("no package with this id: {idx:?}")) } }
    };
}

impl_48!()