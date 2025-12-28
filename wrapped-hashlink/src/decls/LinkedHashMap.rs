macro_rules! deps {
    () => {
        DefaultHashBuilder!();
        Node!();
    };
}

macro_rules! LinkedHashMap {
    () => {
        deps!();
        # [doc = " A version of `HashMap` that has a user controllable order for its entries."] # [doc = ""] # [doc = " It achieves this by keeping its entries in an internal linked list and using a `HashMap` to"] # [doc = " point at nodes in this linked list."] # [doc = ""] # [doc = " The order of entries defaults to \"insertion order\", but the user can also modify the order of"] # [doc = " existing entries by manually moving them to the front or back."] # [doc = ""] # [doc = " There are two kinds of methods that modify the order of the internal list:"] # [doc = ""] # [doc = " * Methods that have names like `to_front` and `to_back` will unsurprisingly move an existing"] # [doc = "   entry to the front or back"] # [doc = " * Methods that have the word `insert` will insert a new entry ot the back of the list, and if"] # [doc = "   that method might replace an entry, that method will *also move that existing entry to the"] # [doc = "   back*."] pub struct LinkedHashMap < K , V , S = DefaultHashBuilder > { table : HashTable < NonNull < Node < K , V > > > , hash_builder : S , values : Option < NonNull < Node < K , V > > > , free : Option < NonNull < Node < K , V > > > , }
    };
}

LinkedHashMap!()