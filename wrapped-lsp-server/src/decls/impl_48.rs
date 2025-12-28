macro_rules! deps {
    () => {
        IoThreads!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl IoThreads { pub fn join (self) -> io :: Result < () > { match self . reader . join () { Ok (r) => r ? , Err (err) => std :: panic :: panic_any (err) , } match self . dropper . join () { Ok (_) => () , Err (err) => { std :: panic :: panic_any (err) ; } } match self . writer . join () { Ok (r) => r , Err (err) => { std :: panic :: panic_any (err) ; } } } }
    };
}

impl_48!()