macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! custom_panic_handler_and_nested_spawn {
    () => {
        deps!();
        # [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn custom_panic_handler_and_nested_spawn () { let (tx , rx) = channel () ; let tx = Mutex :: new (tx) ; let panic_handler = move | e | { tx . lock () . unwrap () . send (e) . unwrap () ; } ; const PANICS : usize = 3 ; let builder = ThreadPoolBuilder :: new () . panic_handler (panic_handler) ; builder . build () . unwrap () . spawn (move | | { for _ in 0 .. PANICS { spawn (move | | { panic ! ("Hello, world!") ; }) ; } }) ; for _ in 0 .. PANICS { let error = rx . recv () . unwrap () ; if let Some (& msg) = error . downcast_ref :: < & str > () { assert_eq ! (msg , "Hello, world!") ; } else { panic ! ("did not receive a string from panic handler") ; } } }
    };
}

custom_panic_handler_and_nested_spawn!();