macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! CoroutineKind {
    () => {
        deps!();
        # [doc = " Describes what kind of coroutine markers, if any, a function has."] # [doc = ""] # [doc = " Coroutine markers are things that cause the function to generate a coroutine, such as `async`,"] # [doc = " which makes the function return `impl Future`, or `gen`, which makes the function return `impl"] # [doc = " Iterator`."] # [derive (Copy , Clone , Encodable , Decodable , Debug , Walkable)] pub enum CoroutineKind { # [doc = " `async`, which returns an `impl Future`."] Async { span : Span , closure_id : NodeId , return_impl_trait_id : NodeId } , # [doc = " `gen`, which returns an `impl Iterator`."] Gen { span : Span , closure_id : NodeId , return_impl_trait_id : NodeId } , # [doc = " `async gen`, which returns an `impl AsyncIterator`."] AsyncGen { span : Span , closure_id : NodeId , return_impl_trait_id : NodeId } , }
    };
}

CoroutineKind!();