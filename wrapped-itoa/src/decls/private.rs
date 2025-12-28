macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { # [doc (hidden)] pub trait Sealed : Copy { # [doc (hidden)] type Buffer : 'static ; fn write (self , buf : & mut Self :: Buffer) -> & str ; } }
    };
}

private!();