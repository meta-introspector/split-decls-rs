macro_rules! Quoted {
    () => {
        struct Quoted < C > (C) ;
    };
}

Quoted!()