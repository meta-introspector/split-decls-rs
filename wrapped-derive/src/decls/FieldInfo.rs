macro_rules! FieldInfo {
    () => {
        # [derive (Clone , Debug)] pub (crate) struct FieldInfo < 'a > { pub accessor : TokenStream2 , pub field : & 'a Field , pub index : usize , }
    };
}

FieldInfo!();