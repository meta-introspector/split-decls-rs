macro_rules! InternError {
    () => {
        # [derive (Debug)] pub enum InternError { BadMutablePointer , DanglingPointer , ConstAllocNotGlobal , PartialPointer , }
    };
}

InternError!()