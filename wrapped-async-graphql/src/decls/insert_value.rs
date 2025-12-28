macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! insert_value {
    () => {
        deps!();
        fn insert_value (target : & mut IndexMap < Name , Value > , name : Name , value : Value) { if let Some (prev_value) = target . get_mut (& name) { if let Value :: Object (target_map) = prev_value { if let Value :: Object (obj) = value { for (key , value) in obj . into_iter () { insert_value (target_map , key , value) ; } } } else if let Value :: List (target_list) = prev_value { if let Value :: List (list) = value { for (idx , value) in list . into_iter () . enumerate () { if let Some (Value :: Object (target_map)) = target_list . get_mut (idx) { if let Value :: Object (obj) = value { for (key , value) in obj . into_iter () { insert_value (target_map , key , value) ; } } } } } } } else { target . insert (name , value) ; } }
    };
}

insert_value!()