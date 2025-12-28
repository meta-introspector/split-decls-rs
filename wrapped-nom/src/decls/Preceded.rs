macro_rules! Preceded {
    () => {
        # [doc = " a"] pub struct Preceded < F , G > { f : F , g : G , }
    };
}

Preceded!()