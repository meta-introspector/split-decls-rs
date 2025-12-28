macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl Stream { # [doc = " Turn ourselves into the underlying byte stream which is a representation of the underlying git tree."] # [doc = ""] # [doc = " Note that the format is unspecified, and its sole use is for transport, not for persistence."] # [doc = " Can be used with [`Self::from_read()`] to decode the contained entries."] pub fn into_read (self) -> impl std :: io :: Read { self . read } # [doc = " Return our internal byte stream from which entries would be generated."] # [doc = ""] # [doc = " Note that the stream must then be consumed in its entirety."] pub fn as_read_mut (& mut self) -> & mut impl std :: io :: Read { self . extra_entries . take () ; & mut self . read } # [doc = " Create a new instance from a stream of bytes in our format."] # [doc = ""] # [doc = " It must have been created from [`Self::into_read()`] to be compatible, and must"] # [doc = " not have been persisted."] pub fn from_read (read : impl std :: io :: Read + 'static) -> Self { Self { read : utils :: Read :: Unknown (Box :: new (read)) , extra_entries : None , path_buf : Some (Vec :: with_capacity (1024) . into ()) , err : Default :: default () , buf : std :: iter :: repeat_n (0 , u16 :: MAX as usize) . collect () , pos : 0 , filled : 0 , } } }
    };
}

impl_30!()