macro_rules! deps {
    () => {
        Error!();
        SymmetricKey!();
        FormatAsPaserk!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        # [cfg (all (feature = "paserk" , feature = "serde"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "paserk" , feature = "serde"))))] impl < V > serde :: Serialize for SymmetricKey < V > where SymmetricKey < V > : FormatAsPaserk , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { use serde :: ser :: Error ; let mut paserk_string = String :: new () ; self . fmt (& mut paserk_string) . map_err (S :: Error :: custom) ? ; serializer . serialize_str (& paserk_string) } }
    };
}

impl_140!();