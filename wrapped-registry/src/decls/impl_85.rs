macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl Drop for Data { fn drop (& mut self) { if ! self . ptr . is_null () { unsafe { HeapFree (GetProcessHeap () , 0 , self . ptr as * mut _) ; } } } }
    };
}

impl_85!();