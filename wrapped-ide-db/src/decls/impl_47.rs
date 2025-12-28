macro_rules! deps {
    () => {
        Definition!();
        Result!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl TryFrom < Definition > for GenericDef { type Error = () ; fn try_from (def : Definition) -> Result < Self , Self :: Error > { match def { Definition :: Function (it) => Ok (it . into ()) , Definition :: Adt (it) => Ok (it . into ()) , Definition :: Trait (it) => Ok (it . into ()) , Definition :: TypeAlias (it) => Ok (it . into ()) , Definition :: SelfType (it) => Ok (it . into ()) , Definition :: Const (it) => Ok (it . into ()) , _ => Err (()) , } } }
    };
}

impl_47!()