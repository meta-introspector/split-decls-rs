macro_rules! powf {
    () => {
        # [inline (always)] # [cfg (all (feature = "std" , feature = "compact"))] pub fn powf (x : f32 , y : f32) -> f32 { x . powf (y) }
    };
}

powf!();