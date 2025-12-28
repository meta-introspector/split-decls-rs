macro_rules! Claims {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone)] # [cfg_attr (feature = "serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] # [doc = " A collection of claims that are passed as payload for a PASETO token."] pub struct Claims { # [cfg_attr (feature = "serde" , serde (flatten))] list_of : HashMap < String , Value > , }
    };
}

Claims!();