macro_rules! deps {
    () => {
        RawEntryMut!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < 'a , K , V , S > RawEntryMut < 'a , K , V , S > { # [doc = " Similarly to `Entry::or_insert`, if this entry is occupied, it will move the existing entry"] # [doc = " to the back of the internal linked list."] # [inline] pub fn or_insert (self , default_key : K , default_val : V) -> (& 'a mut K , & 'a mut V) where K : Hash , S : BuildHasher , { match self { RawEntryMut :: Occupied (mut entry) => { entry . to_back () ; entry . into_key_value () } RawEntryMut :: Vacant (entry) => entry . insert (default_key , default_val) , } } # [doc = " Similarly to `Entry::or_insert_with`, if this entry is occupied, it will move the existing"] # [doc = " entry to the back of the internal linked list."] # [inline] pub fn or_insert_with < F > (self , default : F) -> (& 'a mut K , & 'a mut V) where F : FnOnce () -> (K , V) , K : Hash , S : BuildHasher , { match self { RawEntryMut :: Occupied (mut entry) => { entry . to_back () ; entry . into_key_value () } RawEntryMut :: Vacant (entry) => { let (k , v) = default () ; entry . insert (k , v) } } } # [inline] pub fn and_modify < F > (self , f : F) -> Self where F : FnOnce (& mut K , & mut V) , { match self { RawEntryMut :: Occupied (mut entry) => { { let (k , v) = entry . get_key_value_mut () ; f (k , v) ; } RawEntryMut :: Occupied (entry) } RawEntryMut :: Vacant (entry) => RawEntryMut :: Vacant (entry) , } } }
    };
}

impl_37!()