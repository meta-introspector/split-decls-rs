macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl Rgb { # [doc = " Creates a new [Rgb] color"] # [inline] pub const fn new (r : u8 , g : u8 , b : u8) -> Self { Self { r , g , b } } # [doc = " Creates a new [Rgb] color with a hex code"] # [inline] pub const fn from_hex (hex : u32) -> Self { Self :: new ((hex >> 16) as u8 , (hex >> 8) as u8 , hex as u8) } pub fn from_hex_string (hex : String) -> Self { if hex . chars () . count () == 8 && hex . starts_with ("0x") { let (_ , value_string) = hex . split_at (2) ; let int_val = u64 :: from_str_radix (value_string , 16) ; match int_val { Ok (num) => Self :: new (((num & 0xff0000) >> 16) as u8 , ((num & 0xff00) >> 8) as u8 , (num & 0xff) as u8 ,) , _ => Self :: new (0 , 0 , 0) , } } else { Self :: new (0 , 0 , 0) } } # [doc = " Creates a new [Rgb] color with three [f32] values"] pub fn from_f32 (r : f32 , g : f32 , b : f32) -> Self { Self :: new ((r . clamp (0.0 , 1.0) * 255.0) as u8 , (g . clamp (0.0 , 1.0) * 255.0) as u8 , (b . clamp (0.0 , 1.0) * 255.0) as u8 ,) } # [doc = " Creates a grayscale [Rgb] color"] # [inline] pub const fn gray (x : u8) -> Self { Self :: new (x , x , x) } # [doc = " Creates a grayscale [Rgb] color with a [f32] value"] pub fn gray_f32 (x : f32) -> Self { Self :: from_f32 (x , x , x) } # [doc = " Computes the linear interpolation between `self` and `other` for `t`"] pub fn lerp (& self , other : Self , t : f32) -> Self { let t = t . clamp (0.0 , 1.0) ; self * (1.0 - t) + other * t } }
    };
}

impl_74!();