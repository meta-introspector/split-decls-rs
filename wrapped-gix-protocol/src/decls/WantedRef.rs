macro_rules! WantedRef {
    () => {
        # [doc = " A wanted-ref line received from the server."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct WantedRef { # [doc = " The object id of the wanted ref, as seen by the server."] pub id : gix_hash :: ObjectId , # [doc = " The name of the ref, as requested by the client as a `want-ref` argument."] pub path : BString , }
    };
}

WantedRef!();