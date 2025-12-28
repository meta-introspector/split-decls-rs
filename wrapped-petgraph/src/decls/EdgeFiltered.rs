macro_rules! deps {
    () => {
        FilterEdge!();
    };
}

macro_rules! EdgeFiltered {
    () => {
        deps!();
        # [doc = " An edge-filtering graph adaptor."] # [doc = ""] # [doc = " The adaptor may filter out edges. The filter implements the trait"] # [doc = " `FilterEdge`. Closures of type `Fn(G::EdgeRef) -> bool` already"] # [doc = " implement this trait."] # [doc = ""] # [doc = " The filter may use edge source, target, id, and weight to select whether to"] # [doc = " include the edge or not."] # [derive (Copy , Clone , Debug)] pub struct EdgeFiltered < G , F > (pub G , pub F) ;
    };
}

EdgeFiltered!();