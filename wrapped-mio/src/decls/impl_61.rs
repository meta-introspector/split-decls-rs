macro_rules! deps {
    () => {
        Interest!();
        Registry!();
        Source!();
        Token!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T > Source for Box < T > where T : Source + ? Sized , { fn register (& mut self , registry : & Registry , token : Token , interests : Interest ,) -> io :: Result < () > { (* * self) . register (registry , token , interests) } fn reregister (& mut self , registry : & Registry , token : Token , interests : Interest ,) -> io :: Result < () > { (* * self) . reregister (registry , token , interests) } fn deregister (& mut self , registry : & Registry) -> io :: Result < () > { (* * self) . deregister (registry) } }
    };
}

impl_61!();