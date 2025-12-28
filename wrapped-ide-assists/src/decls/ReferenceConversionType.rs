macro_rules! ReferenceConversionType {
    () => {
        # [derive (Debug)] enum ReferenceConversionType { Copy , AsRefStr , AsRefSlice , Dereferenced , Option , Result , }
    };
}

ReferenceConversionType!();