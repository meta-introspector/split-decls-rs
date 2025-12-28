macro_rules! deps {
    () => {
        FlushOptions!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl FlushOptions { pub fn new () -> FlushOptions { FlushOptions :: default () } # [doc = " Waits until the flush is done."] # [doc = ""] # [doc = " Default: true"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rocksdb::FlushOptions;"] # [doc = ""] # [doc = " let mut options = FlushOptions::default();"] # [doc = " options.set_wait(false);"] # [doc = " ```"] pub fn set_wait (& mut self , wait : bool) { unsafe { ffi :: rocksdb_flushoptions_set_wait (self . inner , c_uchar :: from (wait)) ; } } }
    };
}

impl_201!();