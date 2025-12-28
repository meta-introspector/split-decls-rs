macro_rules! deps {
    () => {
        Transaction!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl Transaction { # [doc = " Creates a new transaction."] pub fn new () -> Result < Self > { let handle = unsafe { CreateTransaction (null_mut () , null_mut () , 0 , 0 , 0 , 0 , null ()) } ; if core :: ptr :: eq (handle , INVALID_HANDLE_VALUE) { Err (Error :: from_thread ()) } else { Ok (Self (handle)) } } # [doc = " Commits the transaction."] # [doc = ""] # [doc = " The transaction rolls back if it is dropped before `commit` is called."] pub fn commit (self) -> Result < () > { let result = unsafe { CommitTransaction (self . 0) } ; if result == 0 { Err (Error :: from_thread ()) } else { Ok (()) } } # [doc = " Constructs a transaction object from an existing handle."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function takes ownership of the handle."] # [doc = " The handle must be owned by the caller and safe to free with `CloseHandle`."] pub unsafe fn from_raw (handle : * mut core :: ffi :: c_void) -> Self { Self (handle) } # [doc = " Returns the underlying transaction handle."] pub fn as_raw (& self) -> * mut core :: ffi :: c_void { self . 0 } }
    };
}

impl_63!()