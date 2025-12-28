macro_rules! deps {
    () => {
        CompiledModules!();
        ExtraBackendMethods!();
        Message!();
    };
}

macro_rules! Coordinator {
    () => {
        deps!();
        pub struct Coordinator < B : ExtraBackendMethods > { sender : Sender < Message < B > > , future : Option < thread :: JoinHandle < Result < CompiledModules , () > > > , phantom : PhantomData < B > , }
    };
}

Coordinator!()