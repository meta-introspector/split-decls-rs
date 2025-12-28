macro_rules! deps {
    () => {
        FixedOutput!();
    };
}

macro_rules! FixedOutputReset {
    () => {
        deps!();
        # [doc = " Trait for hash functions with fixed-size output able to reset themselves."] pub trait FixedOutputReset : FixedOutput + Reset { # [doc = " Write result into provided array and reset the hasher state."] fn finalize_into_reset (& mut self , out : & mut Output < Self >) ; # [doc = " Retrieve result and reset the hasher state."] # [inline] fn finalize_fixed_reset (& mut self) -> Output < Self > { let mut out = Default :: default () ; self . finalize_into_reset (& mut out) ; out } }
    };
}

FixedOutputReset!()