macro_rules! deps {
    () => {
        BOOL!();
    };
}

macro_rules! SECURITY_ATTRIBUTES {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct SECURITY_ATTRIBUTES { pub nLength : u32 , pub lpSecurityDescriptor : * mut core :: ffi :: c_void , pub bInheritHandle : BOOL , }
    };
}

SECURITY_ATTRIBUTES!();