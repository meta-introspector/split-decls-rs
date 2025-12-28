macro_rules! init {
    () => {
        fn init () { static INIT : Once = Once :: new () ; INIT . call_once (| | { openssl_env_init () ; }) ; raw :: init () ; }
    };
}

init!()