macro_rules! deps {
    () => {
        InvalidLength!();
        KeyIvInit!();
        KeyInit!();
        Iv!();
        InnerIvInit!();
        WeakKeyError!();
        Key!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T > KeyIvInit for T where T : InnerIvInit , T :: Inner : KeyInit , { # [inline] fn new (key : & Key < Self > , iv : & Iv < Self >) -> Self { Self :: inner_iv_init (T :: Inner :: new (key) , iv) } # [inline] fn new_from_slices (key : & [u8] , iv : & [u8]) -> Result < Self , InvalidLength > { T :: Inner :: new_from_slice (key) . and_then (| i | T :: inner_iv_slice_init (i , iv)) } # [inline] fn weak_key_test (key : & Key < Self >) -> Result < () , WeakKeyError > { T :: Inner :: weak_key_test (key) } }
    };
}

impl_28!()