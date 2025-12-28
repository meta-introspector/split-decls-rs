macro_rules! Vector3 {
    () => {
        # [repr (C)] # [derive (Clone , Copy , Debug , Default , PartialEq)] pub struct Vector3 { pub X : f32 , pub Y : f32 , pub Z : f32 , }
    };
}

Vector3!();