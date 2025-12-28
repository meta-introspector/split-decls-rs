macro_rules! Matrix4x4 {
    () => {
        # [repr (C)] # [derive (Clone , Copy , Debug , Default , PartialEq)] pub struct Matrix4x4 { pub M11 : f32 , pub M12 : f32 , pub M13 : f32 , pub M14 : f32 , pub M21 : f32 , pub M22 : f32 , pub M23 : f32 , pub M24 : f32 , pub M31 : f32 , pub M32 : f32 , pub M33 : f32 , pub M34 : f32 , pub M41 : f32 , pub M42 : f32 , pub M43 : f32 , pub M44 : f32 , }
    };
}

Matrix4x4!();