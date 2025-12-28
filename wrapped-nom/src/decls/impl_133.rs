macro_rules! deps {
    () => {
        Error!();
        Input!();
        ParserIterator!();
        Parser!();
        Err!();
        State!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < Input , Output , Error , F > core :: iter :: Iterator for ParserIterator < Input , Error , F > where F : Parser < Input , Output = Output , Error = Error > , Input : Clone , { type Item = Output ; fn next (& mut self) -> Option < Self :: Item > { if let State :: Running = self . state . take () . unwrap () { let input = self . input . clone () ; match (self . iterator) . parse (input) { Ok ((i , o)) => { self . input = i ; self . state = Some (State :: Running) ; Some (o) } Err (Err :: Error (_)) => { self . state = Some (State :: Done) ; None } Err (Err :: Failure (e)) => { self . state = Some (State :: Failure (e)) ; None } Err (Err :: Incomplete (i)) => { self . state = Some (State :: Incomplete (i)) ; None } } } else { None } } }
    };
}

impl_133!()