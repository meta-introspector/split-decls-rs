macro_rules! deps {
    () => {
        VswhereInstance!();
        EnumSetupInstances!();
    };
}

macro_rules! VsInstances {
    () => {
        deps!();
        pub enum VsInstances { ComBased (EnumSetupInstances) , VswhereBased (VswhereInstance) , }
    };
}

VsInstances!();