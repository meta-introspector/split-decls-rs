macro_rules! deps {
    () => {
        PasswordHash!();
    };
}

macro_rules! impl_639 {
    () => {
        deps!();
        # [cfg (feature = "serde")] # [cfg_attr (docsrs , doc (cfg (feature = "serde")))] # [doc = " `PasswordHash` serializes as would a [`String`](std::string::String). Note that"] # [doc = " the serialized type likely does not have the same protections that Orion"] # [doc = " provides, such as constant-time operations. A good rule of thumb is to only"] # [doc = " serialize these types for storage. Don't operate on the serialized types."] impl Serialize for PasswordHash { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let encoded_string = self . unprotected_as_encoded () ; serializer . serialize_str (encoded_string) } }
    };
}

impl_639!();