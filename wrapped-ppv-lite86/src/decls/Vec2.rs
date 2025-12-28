macro_rules! Vec2 {
    () => {
        # [doc = " A vector composed of two elements, which may be words or themselves vectors."] pub trait Vec2 < W > { fn extract (self , i : u32) -> W ; fn insert (self , w : W , i : u32) -> Self ; }
    };
}

Vec2!();