macro_rules! deps {
    () => {
        Id!();
        Error!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        # [cfg (all (feature = "paserk" , feature = "serde"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "paserk" , feature = "serde"))))] impl serde :: Serialize for Id { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { use serde :: ser :: Error ; let mut paserk_id = String :: new () ; self . fmt (& mut paserk_id) . map_err (S :: Error :: custom) ? ; serializer . serialize_str (& paserk_id) } }
    };
}

impl_142!()