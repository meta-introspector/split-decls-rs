macro_rules! deps {
    () => {
        Enum!();
        Input!();
        Struct!();
    };
}

macro_rules! try_expand {
    () => {
        deps!();
        fn try_expand (input : & DeriveInput) -> Result < TokenStream > { let input = Input :: from_syn (input) ? ; input . validate () ? ; Ok (match input { Input :: Struct (input) => impl_struct (input) , Input :: Enum (input) => impl_enum (input) , }) }
    };
}

try_expand!();