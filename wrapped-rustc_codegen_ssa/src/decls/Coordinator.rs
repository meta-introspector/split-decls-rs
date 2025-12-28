macro_rules! deps {
    () => {
        Message!();
        CompiledModules!();
        ExtraBackendMethods!();
    };
}

macro_rules! Coordinator {
    () => {
        deps!();
        pub struct Coordinator < B : ExtraBackendMethods > { sender : Sender < Message < B > > , future : Option < thread :: JoinHandle < Result < CompiledModules , () > > > , phantom : PhantomData < B > , }
    };
}

Coordinator!();