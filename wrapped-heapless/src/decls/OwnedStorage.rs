macro_rules! OwnedStorage {
    () => {
        # [doc = " Implementation of [`StringStorage`] that stores the data in an array whose size is known at"] # [doc = " compile time."] pub type OwnedStorage < const N : usize > = OwnedVecStorage < u8 , N > ;
    };
}

OwnedStorage!()