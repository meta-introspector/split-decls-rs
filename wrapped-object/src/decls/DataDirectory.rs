macro_rules! DataDirectory {
    () => {
        # [derive (Default , Clone , Copy)] struct DataDirectory { virtual_address : u32 , size : u32 , }
    };
}

DataDirectory!()