macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! FixedOutput {
    () => {
        deps!();
        # [doc = " Trait for hash functions with fixed-size output."] pub trait FixedOutput : Update + OutputSizeUser + Sized { # [doc = " Consume value and write result into provided array."] fn finalize_into (self , out : & mut Output < Self >) ; # [doc = " Retrieve result and consume the hasher instance."] # [inline] fn finalize_fixed (self) -> Output < Self > { let mut out = Default :: default () ; self . finalize_into (& mut out) ; out } }
    };
}

FixedOutput!();