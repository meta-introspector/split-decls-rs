macro_rules! deps {
    () => {
        StoreMut!();
        Entry!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < 'a , K , V , S > Entry < 'a , K , V , S > where K : Ord , S : StoreMut < K , V > , { # [doc = " Ensures a value is in the entry by inserting the default value if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] pub fn or_insert (self , default : V) -> & 'a mut V { match self { Entry :: Occupied (entry) => entry . into_mut () , Entry :: Vacant (entry) => entry . insert (default) , } } # [doc = " Ensures a value is in the entry by inserting the result of the default function if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] pub fn or_default (self) -> & 'a mut V where V : Default , { self . or_insert (V :: default ()) } # [doc = " Ensures a value is in the entry by inserting the result of the default function if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] pub fn or_insert_with < F : FnOnce () -> V > (self , default : F) -> & 'a mut V { match self { Entry :: Occupied (entry) => entry . into_mut () , Entry :: Vacant (entry) => entry . insert (default ()) , } } # [doc = " Provides in-place mutable access to an occupied entry before any"] # [doc = " potential inserts into the map."] pub fn and_modify < F > (self , f : F) -> Self where F : FnOnce (& mut V) , { match self { Entry :: Occupied (mut entry) => { f (entry . get_mut ()) ; Entry :: Occupied (entry) } Entry :: Vacant (entry) => Entry :: Vacant (entry) , } } }
    };
}

impl_50!();