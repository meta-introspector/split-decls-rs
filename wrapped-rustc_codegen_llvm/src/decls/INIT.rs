macro_rules! INIT {
    () => {
        static INIT : Once = Once :: new () ;
    };
}

INIT!()