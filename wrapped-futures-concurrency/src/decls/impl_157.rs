macro_rules! deps {
    () => {
        FromStream!();
        Consumer!();
        ConsumerState!();
        ConcurrentStream!();
        State!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < S > ConcurrentStream for FromStream < S > where S : Stream , { type Item = S :: Item ; type Future = Ready < Self :: Item > ; async fn drive < C > (self , consumer : C) -> C :: Output where C : Consumer < Self :: Item , Self :: Future > , { let mut iter = pin ! (self . stream) ; let mut consumer = pin ! (consumer) ; loop { let a = async { let item = iter . next () . await ; State :: Item (item) } ; let b = async { let control_flow = consumer . as_mut () . progress () . await ; State :: Progress (control_flow) } ; match (b , a) . race () . await { State :: Progress (control_flow) => match control_flow { ConsumerState :: Break => break , ConsumerState :: Continue => continue , ConsumerState :: Empty => match iter . next () . await { Some (item) => match consumer . as_mut () . send (ready (item)) . await { ConsumerState :: Break => break , ConsumerState :: Empty | ConsumerState :: Continue => continue , } , None => break , } , } , State :: Item (Some (item)) => match consumer . as_mut () . send (ready (item)) . await { ConsumerState :: Break => break , ConsumerState :: Empty | ConsumerState :: Continue => continue , } , State :: Item (None) => break , } } consumer . as_mut () . flush () . await } fn concurrency_limit (& self) -> Option < NonZeroUsize > { None } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
    };
}

impl_157!();