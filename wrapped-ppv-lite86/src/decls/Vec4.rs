macro_rules! Vec4 {
    () => {
        # [doc = " A vector composed of four elements, which may be words or themselves vectors."] pub trait Vec4 < W > { fn extract (self , i : u32) -> W ; fn insert (self , w : W , i : u32) -> Self ; }
    };
}

Vec4!();