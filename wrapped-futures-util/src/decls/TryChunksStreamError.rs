macro_rules! deps {
    () => {
        TryChunksError!();
    };
}

macro_rules! TryChunksStreamError {
    () => {
        deps!();
        type TryChunksStreamError < St > = TryChunksError < < St as TryStream > :: Ok , < St as TryStream > :: Error > ;
    };
}

TryChunksStreamError!()