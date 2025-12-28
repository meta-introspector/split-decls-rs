macro_rules! deps {
    () => {
        SharedState!();
        SerializationSinkInner!();
        SerializationSink!();
        SerializationSinkBuilder!();
        BackingStorage!();
        PageTag!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl SerializationSinkBuilder { pub fn new_from_file (file : fs :: File) -> Result < Self , Box < dyn Error + Send + Sync > > { Ok (Self (SharedState (Arc :: new (Mutex :: new (BackingStorage :: File (file) ,))))) } pub fn new_in_memory () -> SerializationSinkBuilder { Self (SharedState (Arc :: new (Mutex :: new (BackingStorage :: Memory (Vec :: new () ,))))) } pub fn new_sink (& self , page_tag : PageTag) -> SerializationSink { SerializationSink { data : Mutex :: new (SerializationSinkInner { buffer : Vec :: with_capacity (MAX_PAGE_SIZE) , addr : 0 , }) , shared_state : self . 0 . clone () , page_tag , } } }
    };
}

impl_61!();