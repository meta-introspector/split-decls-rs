macro_rules! deps {
    () => {
        OnDiskFileState!();
        OnDiskFile!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T : Clone > OnDiskFile < T > { pub fn path (& self) -> & Path { & self . path } # [doc = " Return true if we hold a memory map of the file already."] pub fn is_loaded (& self) -> bool { matches ! (self . state , OnDiskFileState :: Loaded (_) | OnDiskFileState :: Garbage (_)) } # [doc = " Return true if we are to be collected as garbage"] pub fn is_disposable (& self) -> bool { matches ! (self . state , OnDiskFileState :: Garbage (_) | OnDiskFileState :: Missing) } pub (crate) fn load_strict (& mut self , load : impl FnOnce (& Path) -> std :: io :: Result < T >) -> std :: io :: Result < () > { use OnDiskFileState :: * ; match self . state { Unloaded | Missing => match load (& self . path) { Ok (v) => { self . state = Loaded (v) ; Ok (()) } Err (err) => { self . state = Missing ; Err (err) } } , Loaded (_) | Garbage (_) => Ok (()) , } } # [doc = " If the file is missing, we don't consider this failure but instead return Ok(None) to allow recovery."] # [doc = " when we know that loading is necessary. This also works around borrow check, which is a nice coincidence."] pub fn load_with_recovery (& mut self , load : impl FnOnce (& Path) -> std :: io :: Result < T >) -> std :: io :: Result < Option < T > > { use OnDiskFileState :: * ; match & mut self . state { Loaded (v) | Garbage (v) => Ok (Some (v . clone ())) , Missing => Ok (None) , Unloaded => match load (& self . path) { Ok (v) => { self . state = OnDiskFileState :: Loaded (v . clone ()) ; Ok (Some (v)) } Err (err) if err . kind () == std :: io :: ErrorKind :: NotFound => { self . state = OnDiskFileState :: Missing ; Ok (None) } Err (err) => Err (err) , } , } } pub fn loaded (& self) -> Option < & T > { use OnDiskFileState :: * ; match & self . state { Loaded (v) | Garbage (v) => Some (v) , Unloaded | Missing => None , } } pub fn put_back (& mut self) { match std :: mem :: replace (& mut self . state , OnDiskFileState :: Missing) { OnDiskFileState :: Garbage (v) => self . state = OnDiskFileState :: Loaded (v) , OnDiskFileState :: Missing => self . state = OnDiskFileState :: Unloaded , other @ (OnDiskFileState :: Loaded (_) | OnDiskFileState :: Unloaded) => self . state = other , } } pub fn trash (& mut self) { match std :: mem :: replace (& mut self . state , OnDiskFileState :: Missing) { OnDiskFileState :: Loaded (v) => self . state = OnDiskFileState :: Garbage (v) , other @ (OnDiskFileState :: Garbage (_) | OnDiskFileState :: Unloaded | OnDiskFileState :: Missing) => { self . state = other ; } } } }
    };
}

impl_48!();