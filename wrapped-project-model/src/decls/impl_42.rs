macro_rules! deps {
    () => {
        PackageData!();
        Package!();
        CargoWorkspace!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl ops :: Index < Package > for CargoWorkspace { type Output = PackageData ; fn index (& self , index : Package) -> & PackageData { & self . packages [index] } }
    };
}

impl_42!();