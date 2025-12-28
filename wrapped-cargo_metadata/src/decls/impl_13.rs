macro_rules! deps {
    () => {
        Package!();
        Metadata!();
        PackageId!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'a > std :: ops :: Index < & 'a PackageId > for Metadata { type Output = Package ; fn index (& self , idx : & 'a PackageId) -> & Self :: Output { self . packages . iter () . find (| p | p . id == * idx) . unwrap_or_else (| | panic ! ("no package with this id: {idx:?}")) } }
    };
}

impl_13!()