macro_rules! deps {
    () => {
        DefDatabase!();
    };
}

macro_rules! impl_644 {
    () => {
        deps!();
        # [salsa :: tracked] impl ExternBlockId { # [salsa :: tracked] pub fn abi (self , db : & dyn DefDatabase) -> Option < Symbol > { signatures :: extern_block_abi (db , self) } }
    };
}

impl_644!();