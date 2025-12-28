macro_rules! deps {
    () => {
        AArch64!();
    };
}

macro_rules! Vendor {
    () => {
        deps!();
        # [doc = " Which vendor extensions to support."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum Vendor { # [doc = " A default set of extensions, including some common GNU extensions."] Default , # [doc = " AAarch64 extensions."] AArch64 , }
    };
}

Vendor!();