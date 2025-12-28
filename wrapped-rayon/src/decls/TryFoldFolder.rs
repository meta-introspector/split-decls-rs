macro_rules! TryFoldFolder {
    () => {
        struct TryFoldFolder < 'r , C , U : Try , F > { base : C , fold_op : & 'r F , control : ControlFlow < U :: Residual , U :: Output > , }
    };
}

TryFoldFolder!();