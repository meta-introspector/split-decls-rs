macro_rules! Words4 {
    () => {
        # [doc = " A vector composed of four words; depending on their size, operations may cross lanes."] pub trait Words4 { fn shuffle1230 (self) -> Self ; fn shuffle2301 (self) -> Self ; fn shuffle3012 (self) -> Self ; }
    };
}

Words4!()