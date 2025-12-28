macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < 'a , K , V , const N : usize > Entry < 'a , K , V , N > where K : Eq + Hash , V : Default , { # [doc = " Ensures a value is in the entry by inserting the default value if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # fn main() {"] # [doc = " use heapless::index_map::FnvIndexMap;"] # [doc = ""] # [doc = " let mut book_reviews = FnvIndexMap::<&str, Option<&str>, 16>::new();"] # [doc = ""] # [doc = " book_reviews.entry(\"Pride and Prejudice\").or_default();"] # [doc = ""] # [doc = " assert_eq!(book_reviews[\"Pride and Prejudice\"], None);"] # [doc = " # }"] # [doc = " ```"] # [inline] pub fn or_default (self) -> Result < & 'a mut V , V > { match self { Self :: Occupied (entry) => Ok (entry . into_mut ()) , Self :: Vacant (entry) => entry . insert (Default :: default ()) , } } }
    };
}

impl_94!()