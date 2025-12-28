macro_rules! deps {
    () => {
        Value!();
        Number!();
    };
}

macro_rules! eq_f32 {
    () => {
        deps!();
        fn eq_f32 (value : & Value , other : f32) -> bool { match value { Value :: Number (n) => n . as_f32 () == Some (other) , _ => false , } }
    };
}

eq_f32!()