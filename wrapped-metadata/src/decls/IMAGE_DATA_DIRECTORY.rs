macro_rules! IMAGE_DATA_DIRECTORY {
    () => {
        # [repr (C)] # [derive (Clone , Copy , Default)] pub struct IMAGE_DATA_DIRECTORY { pub VirtualAddress : u32 , pub Size : u32 , }
    };
}

IMAGE_DATA_DIRECTORY!()