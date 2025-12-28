macro_rules! deps {
    () => {
        DashSet!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        # [cfg (feature = "typesize")] impl < K , S > typesize :: TypeSize for DashSet < K , S > where K : typesize :: TypeSize + Eq + Hash , S : typesize :: TypeSize + Clone + BuildHasher , { fn extra_size (& self) -> usize { self . inner . extra_size () } typesize :: if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } } }
    };
}

impl_122!()