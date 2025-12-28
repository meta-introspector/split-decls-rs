macro_rules! Vector2 {
    () => {
        # [repr (C)] # [derive (Clone , Copy , Debug , Default , PartialEq)] pub struct Vector2 { pub X : f32 , pub Y : f32 , }
    };
}

Vector2!()