macro_rules! deps {
    () => {
        Tree!();
        Default!();
        Push!();
    };
}

macro_rules! impl_714 {
    () => {
        deps!();
        impl Push { # [doc = " The `push.default` key"] pub const DEFAULT : Default = Default :: new_with_validate ("default" , & config :: Tree :: PUSH , validate :: Default) ; }
    };
}

impl_714!();