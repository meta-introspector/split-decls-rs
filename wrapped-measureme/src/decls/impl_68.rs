macro_rules! deps {
    () => {
        BackingStorage!();
        SharedState!();
        PageTag!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl SharedState { # [doc = " Copies out the contents of all pages with the given tag and"] # [doc = " concatenates them into a single byte vec. This method is only meant to"] # [doc = " be used for testing and will panic if the underlying backing storage is"] # [doc = " a file instead of in memory."] fn copy_bytes_with_page_tag (& self , page_tag : PageTag) -> Vec < u8 > { let data = self . 0 . lock () ; let data = match * data { BackingStorage :: File (_) => panic ! () , BackingStorage :: Memory (ref data) => data , } ; split_streams (data) . remove (& page_tag) . unwrap_or (Vec :: new ()) } }
    };
}

impl_68!()