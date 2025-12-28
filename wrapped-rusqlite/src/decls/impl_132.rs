macro_rules! deps {
    () => {
        TransactionOperation!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl TransactionOperation { fn from_str (op_str : & str) -> Self { match op_str { "BEGIN" => Self :: Begin , "RELEASE" => Self :: Release , "ROLLBACK" => Self :: Rollback , _ => Self :: Unknown , } } }
    };
}

impl_132!()