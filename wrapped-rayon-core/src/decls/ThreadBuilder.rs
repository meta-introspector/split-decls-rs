macro_rules! deps {
    () => {
        JobRef!();
        Registry!();
    };
}

macro_rules! ThreadBuilder {
    () => {
        deps!();
        # [doc = " Thread builder used for customization via [`ThreadPoolBuilder::spawn_handler()`]."] pub struct ThreadBuilder { name : Option < String > , stack_size : Option < usize > , worker : Worker < JobRef > , stealer : Stealer < JobRef > , registry : Arc < Registry > , index : usize , }
    };
}

ThreadBuilder!()