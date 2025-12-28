macro_rules! deps {
    () => {
        Notification!();
        ExtractError!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Notification { pub fn new (method : String , params : impl serde :: Serialize) -> Notification { Notification { method , params : serde_json :: to_value (params) . unwrap () } } pub fn extract < P : DeserializeOwned > (self , method : & str ,) -> Result < P , ExtractError < Notification > > { if self . method != method { return Err (ExtractError :: MethodMismatch (self)) ; } match serde_json :: from_value (self . params) { Ok (params) => Ok (params) , Err (error) => Err (ExtractError :: JsonError { method : self . method , error }) , } } pub (crate) fn is_exit (& self) -> bool { self . method == "exit" } pub (crate) fn is_initialized (& self) -> bool { self . method == "initialized" } }
    };
}

impl_29!()