macro_rules! Static {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Static { pub (crate) id : StaticId , }
    };
}

Static!()