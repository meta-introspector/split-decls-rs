macro_rules! RustcEnumVariantIdx {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct RustcEnumVariantIdx (pub usize) ;
    };
}

RustcEnumVariantIdx!();