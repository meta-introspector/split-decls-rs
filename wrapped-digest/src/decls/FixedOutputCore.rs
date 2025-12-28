macro_rules! deps {
    () => {
        UpdateCore!();
        BufferKindUser!();
        Buffer!();
    };
}

macro_rules! FixedOutputCore {
    () => {
        deps!();
        # [doc = " Core trait for hash functions with fixed output size."] pub trait FixedOutputCore : UpdateCore + BufferKindUser + OutputSizeUser { # [doc = " Finalize state using remaining data stored in the provided block buffer,"] # [doc = " write result into provided array and leave `self` in a dirty state."] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) ; }
    };
}

FixedOutputCore!();