macro_rules! deps {
    () => {
        Section!();
        RemoteName!();
        Error!();
    };
}

macro_rules! remote_name {
    () => {
        deps!();
        mod remote_name { use std :: borrow :: Cow ; use crate :: { bstr :: { BStr , BString } , config , config :: tree :: { keys :: RemoteName , Section } , } ; impl RemoteName { # [doc = " Create a new instance."] pub const fn new_remote_name (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , super :: validate :: RemoteName) } # [doc = " Try to validate `name` as symbolic remote name and return it."] # [allow (clippy :: result_large_err)] pub fn try_into_symbolic_name (& 'static self , name : Cow < '_ , BStr > ,) -> Result < BString , config :: remote :: symbolic_name :: Error > { crate :: remote :: name :: validated (name . into_owned ()) . map_err (| err | config :: remote :: symbolic_name :: Error :: from (self) . with_source (err)) } } }
    };
}

remote_name!()