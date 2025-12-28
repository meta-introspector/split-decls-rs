macro_rules! powd {
    () => {
        # [inline (always)] # [cfg (all (feature = "std" , feature = "compact"))] pub fn powd (x : f64 , y : f64) -> f64 { x . powf (y) }
    };
}

powd!();