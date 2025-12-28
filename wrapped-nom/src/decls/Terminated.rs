macro_rules! Terminated {
    () => {
        # [doc = " a"] pub struct Terminated < F , G > { f : F , g : G , }
    };
}

Terminated!();