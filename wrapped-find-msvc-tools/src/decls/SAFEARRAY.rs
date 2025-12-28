macro_rules! deps {
    () => {
        ADVANCED_FEATURE_FLAGS!();
        SAFEARRAYBOUND!();
    };
}

macro_rules! SAFEARRAY {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct SAFEARRAY { pub cDims : u16 , pub fFeatures : ADVANCED_FEATURE_FLAGS , pub cbElements : u32 , pub cLocks : u32 , pub pvData : * mut core :: ffi :: c_void , pub rgsabound : [SAFEARRAYBOUND ; 1] , }
    };
}

SAFEARRAY!();