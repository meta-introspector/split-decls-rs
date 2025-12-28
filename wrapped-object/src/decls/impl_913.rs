macro_rules! deps {
    () => {
        ObjectMapEntry!();
        ObjectMap!();
        ObjectMapFile!();
    };
}

macro_rules! impl_913 {
    () => {
        deps!();
        impl < 'data > ObjectMap < 'data > { # [doc = " Get the entry containing the given address."] pub fn get (& self , address : u64) -> Option < & ObjectMapEntry < 'data > > { self . symbols . get (address) . filter (| entry | entry . size == 0 || address . wrapping_sub (entry . address) < entry . size) } # [doc = " Get all symbols in the map."] # [inline] pub fn symbols (& self) -> & [ObjectMapEntry < 'data >] { self . symbols . symbols () } # [doc = " Get all objects in the map."] # [inline] pub fn objects (& self) -> & [ObjectMapFile < 'data >] { & self . objects } }
    };
}

impl_913!()