macro_rules! RustcFieldIdx {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct RustcFieldIdx (pub LocalFieldId) ;
    };
}

RustcFieldIdx!()