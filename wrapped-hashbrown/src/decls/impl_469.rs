macro_rules! deps {
    () => {
        HashTable!();
        RawTable!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl < T > HashTable < T , Global > { # [doc = " Creates an empty `HashTable`."] # [doc = ""] # [doc = " The hash table is initially created with a capacity of 0, so it will not allocate until it"] # [doc = " is first inserted into."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashTable;"] # [doc = " let mut table: HashTable<&str> = HashTable::new();"] # [doc = " assert_eq!(table.len(), 0);"] # [doc = " assert_eq!(table.capacity(), 0);"] # [doc = " ```"] pub const fn new () -> Self { Self { raw : RawTable :: new () , } } # [doc = " Creates an empty `HashTable` with the specified capacity."] # [doc = ""] # [doc = " The hash table will be able to hold at least `capacity` elements without"] # [doc = " reallocating. If `capacity` is 0, the hash table will not allocate."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashTable;"] # [doc = " let mut table: HashTable<&str> = HashTable::with_capacity(10);"] # [doc = " assert_eq!(table.len(), 0);"] # [doc = " assert!(table.capacity() >= 10);"] # [doc = " ```"] pub fn with_capacity (capacity : usize) -> Self { Self { raw : RawTable :: with_capacity (capacity) , } } }
    };
}

impl_469!()