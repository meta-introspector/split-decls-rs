macro_rules! Vector4 {
    () => {
        # [repr (C)] # [derive (Clone , Copy , Debug , Default , PartialEq)] pub struct Vector4 { pub X : f32 , pub Y : f32 , pub Z : f32 , pub W : f32 , }
    };
}

Vector4!()