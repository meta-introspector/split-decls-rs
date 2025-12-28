macro_rules! deps {
    () => {
        Const!();
        Variant!();
        Static!();
        Function!();
    };
}

macro_rules! DefWithBody {
    () => {
        deps!();
        # [doc = " The defs which have a body."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum DefWithBody { Function (Function) , Static (Static) , Const (Const) , Variant (Variant) , }
    };
}

DefWithBody!()