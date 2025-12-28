macro_rules! macro_3 {
    () => {
        impl_is_terminal ! (std :: fs :: File , std :: io :: Stdin , std :: io :: StdinLock <'_ >, std :: io :: Stdout , std :: io :: StdoutLock <'_ >, std :: io :: Stderr , std :: io :: StderrLock <'_ >) ;
    };
}

macro_3!()