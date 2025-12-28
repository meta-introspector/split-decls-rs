macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl Vector2 { pub fn new (X : f32 , Y : f32) -> Self { Self { X , Y } } pub fn zero () -> Self { Self { X : 0f32 , Y : 0f32 } } pub fn one () -> Self { Self { X : 1f32 , Y : 1f32 } } pub fn unit_x () -> Self { Self { X : 1.0 , Y : 0.0 } } pub fn unit_y () -> Self { Self { X : 0.0 , Y : 1.0 } } pub fn dot (& self , rhs : & Self) -> f32 { self . X * rhs . X + self . Y * rhs . Y } pub fn length_squared (& self) -> f32 { self . dot (self) } # [cfg (feature = "std")] pub fn length (& self) -> f32 { self . length_squared () . sqrt () } # [cfg (feature = "std")] pub fn distance (& self , value : & Self) -> f32 { (self - value) . length () } pub fn distance_squared (& self , value : & Self) -> f32 { (self - value) . length_squared () } # [cfg (feature = "std")] pub fn normalize (& self) -> Self { self / self . length () } fn impl_add (& self , rhs : & Self) -> Self { Self { X : self . X + rhs . X , Y : self . Y + rhs . Y , } } fn impl_sub (& self , rhs : & Self) -> Self { Self { X : self . X - rhs . X , Y : self . Y - rhs . Y , } } fn impl_div (& self , rhs : & Self) -> Self { Self { X : self . X / rhs . X , Y : self . Y / rhs . Y , } } fn impl_div_f32 (& self , rhs : f32) -> Self { Self { X : self . X / rhs , Y : self . Y / rhs , } } fn impl_mul (& self , rhs : & Self) -> Self { Self { X : self . X * rhs . X , Y : self . Y * rhs . Y , } } fn impl_mul_f32 (& self , rhs : f32) -> Self { Self { X : self . X * rhs , Y : self . Y * rhs , } } }
    };
}

impl_48!();