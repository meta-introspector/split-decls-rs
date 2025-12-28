macro_rules! ResourceDirectory {
    () => {
        # [doc = " The `.rsrc` section of a PE file."] # [doc = ""] # [doc = " Returned by [`DataDirectories::resource_directory`](super::DataDirectories::resource_directory)."] # [derive (Debug , Clone , Copy)] pub struct ResourceDirectory < 'data > { data : & 'data [u8] , }
    };
}

ResourceDirectory!();