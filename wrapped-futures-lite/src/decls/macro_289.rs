macro_rules! macro_289 {
    () => {
        pin_project ! { # [doc = " Reader for the [`AsyncReadExt::chain()`] method."] pub struct Chain < R1 , R2 > { # [pin] first : R1 , # [pin] second : R2 , done_first : bool , } }
    };
}

macro_289!();