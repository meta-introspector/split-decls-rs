macro_rules! ValueType {
    () => {
        # [doc = " The type of an entry on the DWARF stack."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum ValueType { # [doc = " The generic type, which is address-sized and of unspecified sign,"] # [doc = " as specified in the DWARF 5 standard, section 2.5.1."] # [doc = " This type is also used to represent address base types."] Generic , # [doc = " Signed 8-bit integer type."] I8 , # [doc = " Unsigned 8-bit integer type."] U8 , # [doc = " Signed 16-bit integer type."] I16 , # [doc = " Unsigned 16-bit integer type."] U16 , # [doc = " Signed 32-bit integer type."] I32 , # [doc = " Unsigned 32-bit integer type."] U32 , # [doc = " Signed 64-bit integer type."] I64 , # [doc = " Unsigned 64-bit integer type."] U64 , # [doc = " 32-bit floating point type."] F32 , # [doc = " 64-bit floating point type."] F64 , }
    };
}

ValueType!();