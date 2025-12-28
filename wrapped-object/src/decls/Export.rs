macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! Export {
    () => {
        deps!();
        # [doc = " An exported symbol."] # [doc = ""] # [doc = " Returned by [`Object::exports`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Export < 'data > { name : ByteString < 'data > , address : u64 , }
    };
}

Export!()