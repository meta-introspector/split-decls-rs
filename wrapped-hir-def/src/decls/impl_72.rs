macro_rules! impl_72 {
    () => {
        # [salsa :: tracked] impl ExternBlockId { # [salsa :: tracked] pub fn abi (self , db : & dyn DefDatabase) -> Option < Symbol > { signatures :: extern_block_abi (db , self) } }
    };
}

impl_72!()