macro_rules! Impl {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Impl { pub (crate) id : ImplId , }
    };
}

Impl!()