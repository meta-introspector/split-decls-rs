macro_rules! deps {
    () => {
        TargetGround!();
        Color!();
        Gradient!();
        Rgb!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Gradient { # [doc = " Creates a new [Gradient] with two [Rgb] colors, `start` and `end`"] # [inline] pub const fn new (start : Rgb , end : Rgb) -> Self { Self { start , end } } pub const fn from_color_rgb (start : Color , end : Color) -> Self { let start_grad = match start { Color :: Rgb (r , g , b) => Rgb { r , g , b } , _ => Rgb { r : 0 , g : 0 , b : 0 } , } ; let end_grad = match end { Color :: Rgb (r , g , b) => Rgb { r , g , b } , _ => Rgb { r : 0 , g : 0 , b : 0 } , } ; Self { start : start_grad , end : end_grad , } } # [doc = " Computes the [Rgb] color between `start` and `end` for `t`"] pub fn at (& self , t : f32) -> Rgb { self . start . lerp (self . end , t) } # [doc = " Returns the reverse of `self`"] # [inline] pub const fn reverse (& self) -> Self { Self :: new (self . end , self . start) } pub fn build (& self , text : & str , target : TargetGround) -> String { let delta = 1.0 / text . len () as f32 ; let mut result = text . char_indices () . fold (String :: new () , | mut acc , (i , c) | { let temp = format ! ("\x1B[{}m{}" , self . at (i as f32 * delta) . ansi_color_code (target) , c) ; acc . push_str (& temp) ; acc }) ; result . push_str ("\x1B[0m") ; result } }
    };
}

impl_67!();