macro_rules! deps {
    () => {
        CacheKey!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl PartialEq for CacheKey { fn eq (& self , other : & Self) -> bool { match (self . use_id , other . use_id) { (false , false) => self . location . eq (& other . location) , (true , true) => self . id . eq (& other . id) && self . is_link . eq (& other . is_link) , _ => false , } } }
    };
}

impl_112!();