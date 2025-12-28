macro_rules! delegate_future {
    () => {
        macro_rules ! delegate_future { ($ field : ident) => { fn poll (self : core :: pin :: Pin <& mut Self >, cx : & mut core :: task :: Context <'_ >,) -> core :: task :: Poll < Self :: Output > { self . project () .$ field . poll (cx) } } ; }
    };
}

delegate_future!()