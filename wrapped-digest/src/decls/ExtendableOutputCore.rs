macro_rules! deps {
    () => {
        Buffer!();
        XofReaderCore!();
        UpdateCore!();
        BufferKindUser!();
    };
}

macro_rules! ExtendableOutputCore {
    () => {
        deps!();
        # [doc = " Core trait for hash functions with extendable (XOF) output size."] pub trait ExtendableOutputCore : UpdateCore + BufferKindUser { # [doc = " XOF reader core state."] type ReaderCore : XofReaderCore ; # [doc = " Retrieve XOF reader using remaining data stored in the block buffer"] # [doc = " and leave hasher in a dirty state."] fn finalize_xof_core (& mut self , buffer : & mut Buffer < Self >) -> Self :: ReaderCore ; }
    };
}

ExtendableOutputCore!()