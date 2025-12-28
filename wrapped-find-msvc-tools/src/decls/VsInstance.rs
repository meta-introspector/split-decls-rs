macro_rules! deps {
    () => {
        VswhereInstance!();
        SetupInstance!();
    };
}

macro_rules! VsInstance {
    () => {
        deps!();
        pub enum VsInstance { Com (SetupInstance) , Vswhere (VswhereInstance) , }
    };
}

VsInstance!()