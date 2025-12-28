macro_rules! deps {
    () => {
        RustcEntry!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl < 'a , K , V : Default , A : Allocator > RustcEntry < 'a , K , V , A > { # [doc = " Ensures a value is in the entry by inserting the default value if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # fn main() {"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let mut map: HashMap<&str, Option<u32>> = HashMap::new();"] # [doc = " map.rustc_entry(\"poneyland\").or_default();"] # [doc = ""] # [doc = " assert_eq!(map[\"poneyland\"], None);"] # [doc = " # }"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] pub fn or_default (self) -> & 'a mut V where K : Hash , { match self { Occupied (entry) => entry . into_mut () , Vacant (entry) => entry . insert (Default :: default ()) , } } }
    };
}

impl_374!();