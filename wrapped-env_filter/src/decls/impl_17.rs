macro_rules! deps {
    () => {
        FilterOp!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        # [cfg (feature = "regex")] impl FilterOp { pub (crate) fn new (spec : & str) -> Result < Self , String > { match regex :: Regex :: new (spec) { Ok (r) => Ok (Self { inner : r }) , Err (e) => Err (e . to_string ()) , } } pub (crate) fn is_match (& self , s : & str) -> bool { self . inner . is_match (s) } }
    };
}

impl_17!();