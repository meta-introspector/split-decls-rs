macro_rules! deps {
    () => {
        NTSTATUS!();
    };
}

macro_rules! EXCEPTION_RECORD {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct EXCEPTION_RECORD { pub ExceptionCode : NTSTATUS , pub ExceptionFlags : u32 , pub ExceptionRecord : * mut EXCEPTION_RECORD , pub ExceptionAddress : * mut core :: ffi :: c_void , pub NumberParameters : u32 , pub ExceptionInformation : [usize ; 15] , }
    };
}

EXCEPTION_RECORD!()