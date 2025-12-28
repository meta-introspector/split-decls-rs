macro_rules! deps {
    () => {
        PackageIdSpec!();
    };
}

macro_rules! ProfilePackageSpec {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum ProfilePackageSpec { Spec (PackageIdSpec) , All , }
    };
}

ProfilePackageSpec!();