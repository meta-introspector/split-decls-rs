macro_rules! deps {
    () => {
        Result!();
        Definition!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl TryFrom < DefWithBody > for Definition { type Error = () ; fn try_from (def : DefWithBody) -> Result < Self , Self :: Error > { match def { DefWithBody :: Function (it) => Ok (it . into ()) , DefWithBody :: Static (it) => Ok (it . into ()) , DefWithBody :: Const (it) => Ok (it . into ()) , DefWithBody :: Variant (it) => Ok (it . into ()) , } } }
    };
}

impl_45!()