macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! request_ref {
    () => {
        deps!();
        fn request_ref < 'a , T > (err : & 'a (impl Error + ? Sized)) -> Option < & 'a T > where T : 'static + ? Sized , { error :: request_ref :: < T > (err) }
    };
}

request_ref!()