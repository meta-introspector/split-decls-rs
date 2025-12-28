macro_rules! Matrix3x2 {
    () => {
        # [repr (C)] # [derive (Clone , Copy , Debug , Default , PartialEq)] pub struct Matrix3x2 { pub M11 : f32 , pub M12 : f32 , pub M21 : f32 , pub M22 : f32 , pub M31 : f32 , pub M32 : f32 , }
    };
}

Matrix3x2!();