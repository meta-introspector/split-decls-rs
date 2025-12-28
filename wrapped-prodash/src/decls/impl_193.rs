macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl Value { # [doc = " Returns a number between `Some(0.0)` and `Some(1.0)`, or `None` if the progress is unbounded."] # [doc = ""] # [doc = " A task half done would return `Some(0.5)`."] pub fn fraction (& self) -> Option < f32 > { self . done_at . map (| done_at | self . step . load (Ordering :: SeqCst) as f32 / done_at as f32) } }
    };
}

impl_193!()