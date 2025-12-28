macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! UpdateCore {
    () => {
        deps!();
        # [doc = " Types which consume data in blocks."] pub trait UpdateCore : BlockSizeUser { # [doc = " Update state using the provided data blocks."] fn update_blocks (& mut self , blocks : & [Block < Self >]) ; }
    };
}

UpdateCore!()