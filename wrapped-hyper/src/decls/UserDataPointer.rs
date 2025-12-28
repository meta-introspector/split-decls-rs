macro_rules! UserDataPointer {
    () => {
        # [derive (Clone)] struct UserDataPointer (* mut std :: ffi :: c_void) ;
    };
}

UserDataPointer!()