macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! Import {
    () => {
        deps!();
        # [doc = " An imported symbol."] # [doc = ""] # [doc = " Returned by [`Object::imports`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Import < 'data > { library : ByteString < 'data > , name : ByteString < 'data > , }
    };
}

Import!()