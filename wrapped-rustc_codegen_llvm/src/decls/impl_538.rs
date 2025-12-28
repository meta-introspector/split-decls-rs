macro_rules! deps {
    () => {
        OperandBundle!();
        OperandBundleBox!();
    };
}

macro_rules! impl_538 {
    () => {
        deps!();
        impl < 'a > OperandBundleBox < 'a > { pub (crate) fn new (name : & str , vals : & [& 'a Value]) -> Self { let raw = unsafe { LLVMCreateOperandBundle (name . as_c_char_ptr () , name . len () , vals . as_ptr () , vals . len () as c_uint ,) } ; Self { raw : ptr :: NonNull :: new (raw) . unwrap () } } # [doc = " Dereferences to the underlying `&OperandBundle`."] # [doc = ""] # [doc = " This can't be a `Deref` implementation because `OperandBundle` transitively"] # [doc = " contains an extern type, which is incompatible with `Deref::Target: ?Sized`."] pub (crate) fn as_ref (& self) -> & OperandBundle < 'a > { unsafe { self . raw . as_ref () } } }
    };
}

impl_538!();