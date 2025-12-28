macro_rules! deps {
    () => {
        LOAD_LIBRARY_FLAGS!();
    };
}

macro_rules! LOAD_LIBRARY_AS_DATAFILE {
    () => {
        deps!();
        # [doc = " Map the file into the calling process’ virtual address space as if it were a data file."] # [doc = ""] # [doc = " Nothing is done to execute or prepare to execute the mapped file. Therefore, you cannot call"] # [doc = " functions like [`Library::get`] with this DLL. Using this value causes writes to read-only"] # [doc = " memory to raise an access violation. Use this flag when you want to load a DLL only to extract"] # [doc = " messages or resources from it."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_AS_DATAFILE : LOAD_LIBRARY_FLAGS = 0x00000002 ;
    };
}

LOAD_LIBRARY_AS_DATAFILE!()