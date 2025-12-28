macro_rules! macro_44 {
    () => {
        # [cfg (any (feature = "safe_api" , feature = "alloc" , test))] impl_store_into ! (u64 , to_le_bytes , store_u64_into_le) ;
    };
}

macro_44!()