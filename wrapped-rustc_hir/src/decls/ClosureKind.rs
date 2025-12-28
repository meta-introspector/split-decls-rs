macro_rules! deps {
    () => {
        CoroutineDesugaring!();
        CoroutineKind!();
        Closure!();
    };
}

macro_rules! ClosureKind {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Debug , Copy , Hash , HashStable_Generic , Encodable , Decodable)] pub enum ClosureKind { # [doc = " This is a plain closure expression."] Closure , # [doc = " This is a coroutine expression -- i.e. a closure expression in which"] # [doc = " we've found a `yield`. These can arise either from \"plain\" coroutine"] # [doc = "  usage (e.g. `let x = || { yield (); }`) or from a desugared expression"] # [doc = " (e.g. `async` and `gen` blocks)."] Coroutine (CoroutineKind) , # [doc = " This is a coroutine-closure, which is a special sugared closure that"] # [doc = " returns one of the sugared coroutine (`async`/`gen`/`async gen`). It"] # [doc = " additionally allows capturing the coroutine's upvars by ref, and therefore"] # [doc = " needs to be specially treated during analysis and borrowck."] CoroutineClosure (CoroutineDesugaring) , }
    };
}

ClosureKind!();