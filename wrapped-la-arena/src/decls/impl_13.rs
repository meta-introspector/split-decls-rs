macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'a , IDX , V > Entry < 'a , IDX , V > { # [doc = " Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to"] # [doc = " the value in the entry."] pub fn or_insert (self , default : V) -> & 'a mut V { match self { Self :: Vacant (ent) => ent . insert (default) , Self :: Occupied (ent) => ent . into_mut () , } } # [doc = " Ensures a value is in the entry by inserting the result of the default function if empty, and returns"] # [doc = " a mutable reference to the value in the entry."] pub fn or_insert_with < F : FnOnce () -> V > (self , default : F) -> & 'a mut V { match self { Self :: Vacant (ent) => ent . insert (default ()) , Self :: Occupied (ent) => ent . into_mut () , } } # [doc = " Provides in-place mutable access to an occupied entry before any potential inserts into the map."] pub fn and_modify < F : FnOnce (& mut V) > (mut self , f : F) -> Self { if let Self :: Occupied (ent) = & mut self { f (ent . get_mut ()) ; } self } }
    };
}

impl_13!()