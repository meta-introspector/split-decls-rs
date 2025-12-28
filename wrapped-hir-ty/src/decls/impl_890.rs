macro_rules! deps {
    () => {
        BinOp!();
    };
}

macro_rules! impl_890 {
    () => {
        deps!();
        impl BinOp { fn run_compare < T : PartialEq + PartialOrd > (& self , l : T , r : T) -> bool { match self { BinOp :: Ge => l >= r , BinOp :: Gt => l > r , BinOp :: Le => l <= r , BinOp :: Lt => l < r , BinOp :: Eq => l == r , BinOp :: Ne => l != r , x => panic ! ("`run_compare` called on operator {x:?}") , } } }
    };
}

impl_890!()