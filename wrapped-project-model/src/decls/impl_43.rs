macro_rules! deps {
    () => {
        CargoWorkspace!();
        TargetData!();
        Target!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl ops :: Index < Target > for CargoWorkspace { type Output = TargetData ; fn index (& self , index : Target) -> & TargetData { & self . targets [index] } }
    };
}

impl_43!()