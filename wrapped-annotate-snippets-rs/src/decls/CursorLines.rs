macro_rules! CursorLines {
    () => {
        struct CursorLines < 'a > (& 'a str) ;
    };
}

CursorLines!()