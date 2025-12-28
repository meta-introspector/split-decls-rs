macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! ForLoopKind {
    () => {
        deps!();
        # [doc = " Used to differentiate between `for` loops and `for await` loops."] # [derive (Clone , Copy , Encodable , Decodable , Debug , PartialEq , Eq , Walkable)] pub enum ForLoopKind { For , ForAwait , }
    };
}

ForLoopKind!();