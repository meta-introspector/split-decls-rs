macro_rules! Edition {
    () => {
        # [derive (Serialize , Deserialize , Debug)] pub struct Edition (pub Value) ;
    };
}

Edition!()