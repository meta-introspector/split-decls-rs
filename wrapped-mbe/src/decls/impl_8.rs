macro_rules! deps {
    () => {
        Fragment!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Fragment < '_ > { fn is_empty (& self) -> bool { match self { Fragment :: Empty => true , Fragment :: Tokens (it) => it . len () == 0 , Fragment :: Expr (it) => it . len () == 0 , Fragment :: Path (it) => it . len () == 0 , Fragment :: TokensOwned (it) => it . 0 . is_empty () , } } }
    };
}

impl_8!()