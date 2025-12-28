macro_rules! ArbitrarySelfTypesLevel {
    () => {
        # [doc = " The `arbitrary_self_types_pointers` feature implies `arbitrary_self_types`."] # [derive (Clone , Copy , PartialEq)] enum ArbitrarySelfTypesLevel { Basic , WithPointers , }
    };
}

ArbitrarySelfTypesLevel!()