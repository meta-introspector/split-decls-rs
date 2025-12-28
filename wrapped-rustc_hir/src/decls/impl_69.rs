macro_rules! deps {
    () => {
        PerNS!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < T > PerNS < T > { pub fn map < U , F : FnMut (T) -> U > (self , mut f : F) -> PerNS < U > { PerNS { value_ns : f (self . value_ns) , type_ns : f (self . type_ns) , macro_ns : f (self . macro_ns) } } # [doc = " Note: Do you really want to use this? Often you know which namespace a"] # [doc = " name will belong in, and you can consider just that namespace directly,"] # [doc = " rather than iterating through all of them."] pub fn into_iter (self) -> IntoIter < T , 3 > { [self . value_ns , self . type_ns , self . macro_ns] . into_iter () } # [doc = " Note: Do you really want to use this? Often you know which namespace a"] # [doc = " name will belong in, and you can consider just that namespace directly,"] # [doc = " rather than iterating through all of them."] pub fn iter (& self) -> IntoIter < & T , 3 > { [& self . value_ns , & self . type_ns , & self . macro_ns] . into_iter () } }
    };
}

impl_69!();