macro_rules! deps {
    () => {
        TryReadyChunksError!();
    };
}

macro_rules! TryReadyChunksStreamError {
    () => {
        deps!();
        type TryReadyChunksStreamError < St > = TryReadyChunksError < < St as TryStream > :: Ok , < St as TryStream > :: Error > ;
    };
}

TryReadyChunksStreamError!()