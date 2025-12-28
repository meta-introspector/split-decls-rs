macro_rules! deps {
    () => {
        SerializeMap!();
        Error!();
        Map!();
        Result!();
        Serializer!();
        Value!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , feature = "alloc"))] impl serde :: ser :: Serialize for Map < String , Value > { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeMap ; let mut map = tri ! (serializer . serialize_map (Some (self . len ()))) ; for (k , v) in self { tri ! (map . serialize_entry (k , v)) ; } map . end () } }
    };
}

impl_88!();