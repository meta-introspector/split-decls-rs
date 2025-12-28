macro_rules! OsswuMapParams {
    () => {
        # [doc = " The Optimized Simplified Shallue-van de Woestijne-Ulas parameters"] # [derive (Debug)] pub struct OsswuMapParams < F > where F : Field , { # [doc = " The first constant term"] pub c1 : & 'static [u64] , # [doc = " The second constant term"] pub c2 : F , # [doc = " The ISO A variable or Curve A variable"] pub map_a : F , # [doc = " The ISO A variable or Curve A variable"] pub map_b : F , # [doc = " The Z parameter"] pub z : F , }
    };
}

OsswuMapParams!()