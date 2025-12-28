macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! MapAccess {
    () => {
        deps!();
        struct MapAccess < 'a , R : 'a > { de : & 'a mut Deserializer < R > , first : bool , }
    };
}

MapAccess!()