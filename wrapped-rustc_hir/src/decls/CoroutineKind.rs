macro_rules! deps {
    () => {
        CoroutineSource!();
        CoroutineDesugaring!();
    };
}

macro_rules! CoroutineKind {
    () => {
        deps!();
        # [doc = " The type of source expression that caused this coroutine to be created."] # [derive (Clone , PartialEq , Eq , Debug , Copy , Hash , HashStable_Generic , Encodable , Decodable)] pub enum CoroutineKind { # [doc = " A coroutine that comes from a desugaring."] Desugared (CoroutineDesugaring , CoroutineSource) , # [doc = " A coroutine literal created via a `yield` inside a closure."] Coroutine (Movability) , }
    };
}

CoroutineKind!();