macro_rules! DebugTypeSignature {
    () => {
        # [doc = " A type signature as used in the `.debug_types` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct DebugTypeSignature (pub u64) ;
    };
}

DebugTypeSignature!()