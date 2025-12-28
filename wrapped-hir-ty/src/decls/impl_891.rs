macro_rules! deps {
    () => {
        BinOp!();
    };
}

macro_rules! impl_891 {
    () => {
        deps!();
        impl Display for BinOp { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str (match self { BinOp :: Add => "+" , BinOp :: Sub => "-" , BinOp :: Mul => "*" , BinOp :: Div => "/" , BinOp :: Rem => "%" , BinOp :: BitXor => "^" , BinOp :: BitAnd => "&" , BinOp :: BitOr => "|" , BinOp :: Shl => "<<" , BinOp :: Shr => ">>" , BinOp :: Eq => "==" , BinOp :: Lt => "<" , BinOp :: Le => "<=" , BinOp :: Ne => "!=" , BinOp :: Ge => ">=" , BinOp :: Gt => ">" , BinOp :: Offset => "`offset`" , }) } }
    };
}

impl_891!();