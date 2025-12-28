macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a , K , V , S > Entry < 'a , K , V , S > { # [doc = " If this entry is vacant, inserts a new entry with the given value and returns a reference to"] # [doc = " it."] # [doc = ""] # [doc = " If this entry is occupied, this method *moves the occupied entry to the back of the internal"] # [doc = " linked list* and returns a reference to the existing value."] # [inline] pub fn or_insert (self , default : V) -> & 'a mut V where K : Hash , S : BuildHasher , { match self { Entry :: Occupied (mut entry) => { entry . to_back () ; entry . into_mut () } Entry :: Vacant (entry) => entry . insert (default) , } } # [doc = " Similar to `Entry::or_insert`, but accepts a function to construct a new value if this entry"] # [doc = " is vacant."] # [inline] pub fn or_insert_with < F : FnOnce () -> V > (self , default : F) -> & 'a mut V where K : Hash , S : BuildHasher , { match self { Entry :: Occupied (mut entry) => { entry . to_back () ; entry . into_mut () } Entry :: Vacant (entry) => entry . insert (default ()) , } } # [inline] pub fn key (& self) -> & K { match * self { Entry :: Occupied (ref entry) => entry . key () , Entry :: Vacant (ref entry) => entry . key () , } } # [inline] pub fn and_modify < F > (self , f : F) -> Self where F : FnOnce (& mut V) , { match self { Entry :: Occupied (mut entry) => { f (entry . get_mut ()) ; Entry :: Occupied (entry) } Entry :: Vacant (entry) => Entry :: Vacant (entry) , } } }
    };
}

impl_25!()