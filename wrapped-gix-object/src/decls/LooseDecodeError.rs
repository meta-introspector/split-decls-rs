macro_rules! deps {
    () => {
        Error!();
        Object!();
    };
}

macro_rules! LooseDecodeError {
    () => {
        deps!();
        # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum LooseDecodeError { # [error (transparent)] InvalidHeader (# [from] LooseHeaderDecodeError) , # [error (transparent)] InvalidContent (# [from] DecodeError) , # [error ("Object sized {size} does not fit into memory - this can happen on 32 bit systems")] OutOfMemory { size : u64 } , }
    };
}

LooseDecodeError!();