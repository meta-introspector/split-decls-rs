macro_rules! Service {
    () => {
        # [doc = " The kind of service to invoke on the client or the server side."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Service { # [doc = " The service sending packs from a server to the client. Used for fetching pack data."] UploadPack , # [doc = " The service receiving packs produced by the client, who sends a pack to the server."] ReceivePack , }
    };
}

Service!()