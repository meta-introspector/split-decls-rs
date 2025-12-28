macro_rules! deps {
    () => {
        PackageId!();
        Resolve!();
        Node!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'a > std :: ops :: Index < & 'a PackageId > for Resolve { type Output = Node ; fn index (& self , idx : & 'a PackageId) -> & Self :: Output { self . nodes . iter () . find (| p | p . id == * idx) . unwrap_or_else (| | panic ! ("no Node with this id: {idx:?}")) } }
    };
}

impl_53!();